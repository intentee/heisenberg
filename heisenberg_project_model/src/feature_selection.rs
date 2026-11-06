use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::rc::Rc;

use cargo::resolver::features::CliFeatures;
use cargo::util::interning::InternedString;
use cargo::workspace::FeatureValue;
use ra_ap_paths::AbsPath;

use crate::command_line_feature_scope::CommandLineFeatureScope;
use crate::package_document::PackageDocument;
use crate::project_model_error::ProjectModelError;
use crate::resolved_packages::ResolvedPackages;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FeatureSelection {
    All,
    Listed {
        default_features: bool,
        features: BTreeSet<String>,
    },
}

impl FeatureSelection {
    /// # Errors
    ///
    /// Returns [`ProjectModelError::SelectedFeaturesRejected`] when cargo cannot read `requested`
    /// as a feature list, and [`ProjectModelError::SelectedFeatureNotAName`] for a requested
    /// feature that cargo reads as something other than a feature name.
    pub fn listed(default_features: bool, requested: &[String]) -> Result<Self, ProjectModelError> {
        CliFeatures::from_command_line(requested, false, default_features)
            .map_err(|source| ProjectModelError::SelectedFeaturesRejected(source.into()))
            .and_then(|CliFeatures { features, .. }| {
                features
                    .iter()
                    .map(|value| match value {
                        FeatureValue::Feature(name) => Ok(name.to_string()),
                        FeatureValue::Dep { .. } | FeatureValue::DepFeature { .. } => {
                            Err(ProjectModelError::SelectedFeatureNotAName {
                                feature: value.to_string(),
                            })
                        }
                    })
                    .collect()
            })
            .map(|features| Self::Listed {
                default_features,
                features,
            })
    }

    #[must_use]
    pub fn cli_features(&self) -> CliFeatures {
        match self {
            Self::All => CliFeatures::new_all(true),
            Self::Listed {
                default_features,
                features,
            } => CliFeatures {
                all_features: false,
                features: Rc::new(
                    features
                        .iter()
                        .map(|feature| FeatureValue::Feature(InternedString::new(feature)))
                        .collect(),
                ),
                uses_default_features: *default_features,
            },
        }
    }

    /// # Errors
    ///
    /// Returns [`ProjectModelError::SelectedFeaturesReachOnlyTheRootPackage`] when cargo would
    /// apply the selection to the root package alone while the workspace has other members, and
    /// [`ProjectModelError::UnknownSelectedFeature`] for a selected feature that no workspace
    /// member defines.
    pub fn validate(
        &self,
        resolved: &ResolvedPackages,
        manifest_path: &AbsPath,
    ) -> Result<(), ProjectModelError> {
        match self {
            Self::All => Ok(()),
            Self::Listed { .. }
                if resolved.command_line_features == CommandLineFeatureScope::RootPackage
                    && resolved.members().nth(1).is_some() =>
            {
                Err(ProjectModelError::SelectedFeaturesReachOnlyTheRootPackage {
                    manifest_path: manifest_path.to_path_buf(),
                })
            }
            Self::Listed { features, .. } => {
                let defined: BTreeSet<&String> = resolved
                    .members()
                    .flat_map(|PackageDocument { metadata, .. }| metadata.features.keys())
                    .collect();

                features
                    .iter()
                    .find(|feature| !defined.contains(feature))
                    .map_or(Ok(()), |feature| {
                        Err(ProjectModelError::UnknownSelectedFeature {
                            feature: feature.clone(),
                            manifest_path: manifest_path.to_path_buf(),
                        })
                    })
            }
        }
    }
}

impl Display for FeatureSelection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => formatter.write_str("all"),
            Self::Listed {
                default_features,
                features,
            } => {
                let parts: Vec<&str> = default_features
                    .then_some("default features")
                    .into_iter()
                    .chain(features.iter().map(String::as_str))
                    .collect();

                if parts.is_empty() {
                    formatter.write_str("none")
                } else {
                    formatter.write_str(&parts.join(", "))
                }
            }
        }
    }
}

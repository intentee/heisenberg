use std::collections::BTreeSet;

use ra_ap_paths::AbsPath;

use crate::command_line_feature_scope::CommandLineFeatureScope;
use crate::configuration::Configuration;
use crate::constants::SUPPORTED_TARGETS;
use crate::declared_configuration::DeclaredConfiguration;
use crate::feature_selection::FeatureSelection;
use crate::package_document::PackageDocument;
use crate::project_model_error::ProjectModelError;
use crate::resolved_packages::ResolvedPackages;

pub struct DeclaredConfigurations<'declaration> {
    pub manifest_path: &'declaration AbsPath,
    pub resolved: &'declaration ResolvedPackages,
}

impl DeclaredConfigurations<'_> {
    pub fn validated(
        &self,
        declared: Vec<DeclaredConfiguration>,
    ) -> Result<BTreeSet<Configuration>, ProjectModelError> {
        if declared.is_empty() {
            return Err(ProjectModelError::NoConfigurations {
                manifest_path: self.manifest_path.to_path_buf(),
            });
        }

        let defined: BTreeSet<&String> = self
            .resolved
            .members()
            .flat_map(|PackageDocument { metadata, .. }| metadata.features.keys())
            .collect();
        let mut validated: BTreeSet<Configuration> = BTreeSet::new();

        for (index, declaration) in declared.into_iter().enumerate() {
            let configuration = self.configuration(declaration, index, &defined)?;

            if validated
                .iter()
                .any(|Configuration { name, .. }| *name == configuration.name)
            {
                return Err(ProjectModelError::ConfigurationDeclaredTwice {
                    configuration: configuration.name,
                    manifest_path: self.manifest_path.to_path_buf(),
                });
            }

            if let Some(analysing_the_same_build) = validated.iter().find(
                |Configuration {
                     features, target, ..
                 }| {
                    *target == configuration.target && *features == configuration.features
                },
            ) {
                return Err(ProjectModelError::ConfigurationsAnalyseTheSameBuild {
                    first: analysing_the_same_build.name.clone(),
                    manifest_path: self.manifest_path.to_path_buf(),
                    second: configuration.name,
                });
            }

            validated.insert(configuration);
        }

        Ok(validated)
    }

    fn configuration(
        &self,
        DeclaredConfiguration {
            default_features,
            features,
            name,
            target,
        }: DeclaredConfiguration,
        index: usize,
        defined: &BTreeSet<&String>,
    ) -> Result<Configuration, ProjectModelError> {
        if name.is_empty() {
            return Err(ProjectModelError::ConfigurationWithoutName {
                index,
                manifest_path: self.manifest_path.to_path_buf(),
            });
        }

        let supported = SUPPORTED_TARGETS
            .into_iter()
            .find(|supported| supported.triple() == target)
            .ok_or_else(|| ProjectModelError::UnknownConfigurationTarget {
                configuration: name.clone(),
                manifest_path: self.manifest_path.to_path_buf(),
                target,
            })?;
        let mut selected = BTreeSet::new();

        for feature in features {
            if !defined.contains(&feature) {
                return Err(ProjectModelError::UnknownConfigurationFeature {
                    configuration: name,
                    feature,
                    manifest_path: self.manifest_path.to_path_buf(),
                });
            }

            if !selected.insert(feature.clone()) {
                return Err(ProjectModelError::ConfigurationFeatureListedTwice {
                    configuration: name,
                    feature,
                    manifest_path: self.manifest_path.to_path_buf(),
                });
            }
        }

        let features = FeatureSelection {
            default_features,
            features: selected,
        };

        if !features.is_default()
            && self.resolved.command_line_features == CommandLineFeatureScope::RootPackage
            && self.resolved.members().nth(1).is_some()
        {
            return Err(
                ProjectModelError::ConfigurationFeaturesReachOnlyTheRootPackage {
                    configuration: name,
                    manifest_path: self.manifest_path.to_path_buf(),
                },
            );
        }

        Ok(Configuration {
            name,
            target: supported,
            features,
        })
    }
}

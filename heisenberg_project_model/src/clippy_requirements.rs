use std::collections::BTreeMap;
use std::collections::BTreeSet;

use toml::Value;

use crate::clippy_configuration_file::ClippyConfigurationFile;
use crate::constants::PANICKING_LINTS;
use crate::constants::REQUIRED_CLIPPY_SETTINGS;
use crate::constants::REQUIRED_LINT_GROUPS;
use crate::constants::RESTRICTING_CLIPPY_SETTINGS;
use crate::lint_level::LintLevel;
use crate::package_document::PackageDocument;
use crate::package_violation::PackageViolation;
use crate::project_model_error::ProjectModelError;
use crate::required_clippy_setting::RequiredClippySetting;
use crate::resolved_packages::ResolvedPackages;

fn member_violations(
    member: &PackageDocument,
    configuration: ClippyConfigurationFile,
    resolved: &ResolvedPackages,
) -> Vec<PackageViolation> {
    let workspace_path = &resolved.workspace_path();
    let PackageDocument {
        lints, metadata, ..
    } = member;
    let name = metadata.name.to_string();
    let manifest = member.relative_manifest(workspace_path);
    let table = member.lint_table(workspace_path);
    let mut violations: Vec<PackageViolation> = REQUIRED_LINT_GROUPS
        .iter()
        .chain(PANICKING_LINTS.iter())
        .filter(|lint| !LintLevel { lint, lints }.is_denied())
        .map(|lint| PackageViolation::RequiredLintNotDenied {
            lint: lint.path(),
            table: table.clone(),
        })
        .collect();

    match configuration {
        ClippyConfigurationFile::Ambiguous { file } => {
            violations.push(PackageViolation::ClippyConfigurationAmbiguous {
                file: workspace_path.relative(&file),
            });
        }
        ClippyConfigurationFile::Missing => {
            violations.push(PackageViolation::ClippyConfigurationMissing {
                manifest,
                package: name,
            });
        }
        ClippyConfigurationFile::Present { file, settings } => {
            let relative = workspace_path.relative(&file);

            violations.extend(
                REQUIRED_CLIPPY_SETTINGS
                    .iter()
                    .filter(|RequiredClippySetting { name, value }| {
                        settings.get(*name) != Some(&Value::Boolean(*value))
                    })
                    .map(|RequiredClippySetting { name, value }| {
                        PackageViolation::RequiredClippySettingMissing {
                            file: relative.clone(),
                            setting: (*name).to_owned(),
                            value: *value,
                        }
                    }),
            );
            violations.extend(
                settings
                    .keys()
                    .filter(|setting| {
                        !REQUIRED_CLIPPY_SETTINGS
                            .iter()
                            .any(|RequiredClippySetting { name, .. }| name == setting)
                            && !RESTRICTING_CLIPPY_SETTINGS.contains(&setting.as_str())
                    })
                    .map(|setting| PackageViolation::UnvettedClippySetting {
                        file: relative.clone(),
                        setting: setting.clone(),
                    }),
            );
        }
    }

    violations
}

pub struct ClippyRequirements<'requirements> {
    pub resolved: &'requirements ResolvedPackages,
}

impl ClippyRequirements<'_> {
    /// # Errors
    ///
    /// Returns [`ProjectModelError`] when a clippy configuration that discovery finds
    /// cannot be read or is not valid TOML.
    pub fn violations(&self) -> Result<Vec<PackageViolation>, ProjectModelError> {
        let workspace_path = self.resolved.workspace_path();
        let mut found = BTreeMap::new();

        self.resolved
            .members()
            .map(|package| {
                ClippyConfigurationFile::discover(
                    &package.metadata.manifest_path,
                    workspace_path.root,
                    &mut found,
                )
                .map(|configuration| member_violations(package, configuration, self.resolved))
            })
            .collect::<Result<Vec<Vec<PackageViolation>>, ProjectModelError>>()
            .map(|violations| {
                violations
                    .into_iter()
                    .flatten()
                    .collect::<BTreeSet<PackageViolation>>()
                    .into_iter()
                    .collect()
            })
    }
}

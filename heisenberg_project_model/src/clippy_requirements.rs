use std::collections::BTreeSet;

use toml::Value;

use crate::clippy_configuration_file::ClippyConfigurationFile;
use crate::clippy_lint_level::ClippyLintLevel;
use crate::constants::CLIPPY_DENIED_LINTS;
use crate::constants::CLIPPY_TEST_ALLOWANCES;
use crate::package_document::PackageDocument;
use crate::package_violation::PackageViolation;
use crate::project_model_error::ProjectModelError;
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
    let may_panic = resolved.is_test_crate(member);
    let mut violations: Vec<PackageViolation> = CLIPPY_DENIED_LINTS
        .iter()
        .filter(|_| !may_panic)
        .filter(|lint| !ClippyLintLevel { lint, lints }.is_denied())
        .map(|lint| PackageViolation::ClippyLintNotDenied {
            lint: lint.name.to_owned(),
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
            violations.extend(
                CLIPPY_TEST_ALLOWANCES
                    .iter()
                    .filter(|setting| {
                        !matches!(settings.get(**setting), Some(Value::Boolean(true)))
                    })
                    .map(|setting| PackageViolation::ClippyTestAllowanceMissing {
                        file: workspace_path.relative(&file),
                        setting: (*setting).to_owned(),
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

        self.resolved
            .members()
            .map(|package| {
                ClippyConfigurationFile::discover(
                    &package.metadata.manifest_path,
                    workspace_path.root,
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

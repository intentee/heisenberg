use cargo_util_schemas::manifest::TomlLintLevel;

use crate::package_document::PackageDocument;
use crate::package_violation::PackageViolation;
use crate::resolved_packages::ResolvedPackages;

pub struct ManifestLintAllowances<'allowances> {
    pub resolved: &'allowances ResolvedPackages,
}

impl ManifestLintAllowances<'_> {
    #[must_use]
    pub fn violations(&self) -> Vec<PackageViolation> {
        let workspace_path = self.resolved.workspace_path();

        self.resolved
            .members()
            .flat_map(|member| {
                let PackageDocument {
                    lints, metadata, ..
                } = member;
                let manifest = member.relative_manifest(&workspace_path);

                lints.iter().flat_map(move |(tool, tool_lints)| {
                    let manifest = manifest.clone();

                    tool_lints
                        .iter()
                        .filter(|(_, lint)| lint.level() == TomlLintLevel::Allow)
                        .map(move |(lint, _)| PackageViolation::LintAllowedInManifest {
                            lint: lint.clone(),
                            manifest: manifest.clone(),
                            package: metadata.name.to_string(),
                            tool: tool.clone(),
                        })
                })
            })
            .collect()
    }
}

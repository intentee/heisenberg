use std::collections::BTreeSet;

use cargo_util_schemas::manifest::TomlLintLevel;

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
                let table = member.lint_table(&workspace_path);

                member.lints.iter().flat_map(move |(tool, tool_lints)| {
                    let table = table.clone();

                    tool_lints
                        .iter()
                        .filter(|(_, lint)| lint.level() == TomlLintLevel::Allow)
                        .map(move |(lint, _)| PackageViolation::LintAllowedInManifest {
                            lint: lint.clone(),
                            table: table.clone(),
                            tool: tool.clone(),
                        })
                })
            })
            .collect::<BTreeSet<PackageViolation>>()
            .into_iter()
            .collect()
    }
}

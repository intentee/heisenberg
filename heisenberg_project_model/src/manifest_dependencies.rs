use std::collections::BTreeMap;

use cargo_util_schemas::manifest::InheritableDependency;
use cargo_util_schemas::manifest::PackageName;
use cargo_util_schemas::manifest::TomlDependency;
use cargo_util_schemas::manifest::TomlManifest;
use cargo_util_schemas::manifest::TomlPlatform;
use cargo_util_schemas::manifest::TomlWorkspace;

use crate::dependency_requirement::DependencyRequirement;
use crate::unpinned_dependency::UnpinnedDependency;

fn unpinned(name: &PackageName, dependency: &TomlDependency) -> Option<UnpinnedDependency> {
    DependencyRequirement { dependency }
        .unpinned()
        .map(|requirement| UnpinnedDependency {
            dependency: name.to_string(),
            requirement,
        })
}

fn unpinned_in_a_table(
    table: Option<&BTreeMap<PackageName, InheritableDependency>>,
) -> Vec<UnpinnedDependency> {
    table
        .into_iter()
        .flatten()
        .filter_map(|(name, dependency)| match dependency {
            InheritableDependency::Value(written) => unpinned(name, written),
            InheritableDependency::Inherit(_) => None,
        })
        .collect()
}

pub struct ManifestDependencies<'manifest> {
    pub manifest: &'manifest TomlManifest,
}

impl ManifestDependencies<'_> {
    #[must_use]
    pub fn unpinned(&self) -> Vec<UnpinnedDependency> {
        let manifest = self.manifest;

        [
            unpinned_in_a_table(manifest.dependencies.as_ref()),
            unpinned_in_a_table(manifest.dev_dependencies()),
            unpinned_in_a_table(manifest.build_dependencies()),
        ]
        .into_iter()
        .chain(manifest.target.iter().flat_map(BTreeMap::values).flat_map(
            |platform: &TomlPlatform| {
                [
                    unpinned_in_a_table(platform.dependencies.as_ref()),
                    unpinned_in_a_table(platform.dev_dependencies()),
                    unpinned_in_a_table(platform.build_dependencies()),
                ]
            },
        ))
        .flatten()
        .chain(
            manifest
                .workspace
                .iter()
                .filter_map(|TomlWorkspace { dependencies, .. }| dependencies.as_ref())
                .flatten()
                .filter_map(|(name, dependency)| unpinned(name, dependency)),
        )
        .collect()
    }
}

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
        .chain(
            manifest
                .patch
                .iter()
                .flat_map(BTreeMap::values)
                .flatten()
                .filter_map(|(name, dependency)| unpinned(name, dependency)),
        )
        .chain(
            manifest
                .replace
                .iter()
                .flatten()
                .filter_map(|(package, dependency)| {
                    DependencyRequirement { dependency }
                        .unpinned()
                        .map(|requirement| UnpinnedDependency {
                            dependency: package.clone(),
                            requirement,
                        })
                }),
        )
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use cargo_util_schemas::manifest::TomlManifest;
    use toml::from_str;

    use super::ManifestDependencies;
    use crate::unpinned_dependency::UnpinnedDependency;
    use crate::unpinned_requirement::UnpinnedRequirement;

    fn unpinned(written: &str) -> Vec<UnpinnedDependency> {
        let manifest = from_str::<TomlManifest>(written)
            .expect("the manifest of a test is a valid cargo manifest");

        ManifestDependencies {
            manifest: &manifest,
        }
        .unpinned()
    }

    #[test]
    fn patch_that_follows_a_branch_is_not_pinned() {
        assert_eq!(
            unpinned(
                "[patch.crates-io]\npatched = { git = \"https://example.invalid/patched\", branch = \"main\" }"
            ),
            vec![UnpinnedDependency {
                dependency: "patched".to_owned(),
                requirement: UnpinnedRequirement::GitWithoutRevision,
            }]
        );
    }

    #[test]
    fn replacement_that_follows_a_branch_is_not_pinned() {
        assert_eq!(
            unpinned(
                "[replace]\n\"replaced:1.0.0\" = { git = \"https://example.invalid/replaced\", branch = \"main\" }"
            ),
            vec![UnpinnedDependency {
                dependency: "replaced:1.0.0".to_owned(),
                requirement: UnpinnedRequirement::GitWithoutRevision,
            }]
        );
    }

    #[test]
    fn patch_at_a_revision_is_pinned() {
        assert_eq!(
            unpinned(
                "[patch.crates-io]\npatched = { git = \"https://example.invalid/patched\", rev = \"0123abc\" }"
            ),
            Vec::new()
        );
    }
}

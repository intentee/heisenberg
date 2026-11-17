use crate::manifest_violation::ManifestViolation;
use crate::package_document::PackageDocument;
use crate::package_violation::PackageViolation;
use crate::resolved_packages::ResolvedPackages;
use crate::unpinned_dependency::UnpinnedDependency;

fn violations_in(manifest: &str, unpinned: &[UnpinnedDependency]) -> Vec<PackageViolation> {
    unpinned
        .iter()
        .map(
            |UnpinnedDependency {
                 dependency,
                 requirement,
             }| {
                PackageViolation::Manifest(ManifestViolation::UnpinnedDependency {
                    dependency: dependency.clone(),
                    manifest: manifest.to_owned(),
                    requirement: requirement.clone(),
                })
            },
        )
        .collect()
}

pub struct DependencyRequirements<'requirements> {
    pub resolved: &'requirements ResolvedPackages,
}

impl DependencyRequirements<'_> {
    #[must_use]
    pub fn violations(&self) -> Vec<PackageViolation> {
        let workspace_path = self.resolved.workspace_path();
        let root_manifest = workspace_path.manifest();

        violations_in(
            &root_manifest,
            &self.resolved.workspace.unpinned_dependencies,
        )
        .into_iter()
        .chain(
            self.resolved
                .members()
                .map(|member| {
                    let PackageDocument {
                        unpinned_dependencies,
                        ..
                    } = member;

                    violations_in(
                        &member.relative_manifest(&workspace_path),
                        unpinned_dependencies,
                    )
                })
                .filter(|violations| {
                    violations
                        .iter()
                        .all(|violation| violation.file() != root_manifest)
                })
                .flatten(),
        )
        .collect()
    }
}

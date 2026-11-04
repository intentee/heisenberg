use std::collections::BTreeSet;

use cargo::workspace::dependency::DepKind;

use crate::dependency_document::DependencyDocument;
use crate::dependency_kind_document::DependencyKindDocument;
use crate::package_document::PackageDocument;
use crate::package_violation::PackageViolation;
use crate::resolved_packages::ResolvedPackages;

fn production_dependents<'resolved>(
    resolved: &'resolved ResolvedPackages,
    test_crates: &BTreeSet<&str>,
    PackageDocument { metadata, .. }: &PackageDocument,
) -> Vec<&'resolved PackageDocument> {
    resolved
        .members()
        .filter(
            |PackageDocument {
                 dependencies,
                 metadata: dependent,
                 ..
             }| {
                !test_crates.contains(dependent.name.as_str())
                    && dependencies.iter().any(
                        |DependencyDocument {
                             kinds, package_id, ..
                         }| {
                            *package_id == metadata.id.repr
                                && kinds.iter().any(|DependencyKindDocument { kind, .. }| {
                                    *kind != DepKind::Development
                                })
                        },
                    )
            },
        )
        .collect()
}

pub struct DeclaredTestCrates {
    pub names: Vec<String>,
}

impl DeclaredTestCrates {
    #[must_use]
    pub fn test_crates(&self, resolved: &ResolvedPackages) -> BTreeSet<String> {
        self.valid(resolved)
            .into_iter()
            .map(str::to_owned)
            .collect()
    }

    #[must_use]
    pub fn violations(&self, resolved: &ResolvedPackages) -> Vec<PackageViolation> {
        let workspace_path = resolved.workspace_path();
        let test_crates = self.valid(resolved);
        let mut declared: BTreeSet<&str> = BTreeSet::new();
        let mut violations: BTreeSet<PackageViolation> = BTreeSet::new();

        for name in &self.names {
            if !declared.insert(name) {
                violations.insert(PackageViolation::TestCrateDeclaredTwice {
                    manifest: workspace_path.manifest(),
                    package: name.clone(),
                });
            }
        }

        for name in declared {
            match resolved
                .members()
                .find(|PackageDocument { metadata, .. }| metadata.name.as_str() == name)
            {
                None => {
                    violations.insert(PackageViolation::TestCrateNotAMember {
                        manifest: workspace_path.manifest(),
                        package: name.to_owned(),
                    });
                }
                Some(member) if !member.is_unpublished() => {
                    violations.insert(PackageViolation::PublishableTestCrate {
                        manifest: member.relative_manifest(&workspace_path),
                        package: name.to_owned(),
                    });
                }
                Some(member) => {
                    violations.extend(
                        production_dependents(resolved, &test_crates, member)
                            .into_iter()
                            .map(|dependent| PackageViolation::TestCrateUsedByProduction {
                                dependent: dependent.metadata.name.to_string(),
                                manifest: dependent.relative_manifest(&workspace_path),
                                package: name.to_owned(),
                            }),
                    );
                }
            }
        }

        violations.into_iter().collect()
    }

    fn valid<'resolved>(&self, resolved: &'resolved ResolvedPackages) -> BTreeSet<&'resolved str> {
        let mut test_crates: BTreeSet<&str> = resolved
            .members()
            .filter(|member| {
                member.is_unpublished()
                    && self
                        .names
                        .iter()
                        .any(|name| name.as_str() == member.metadata.name.as_str())
            })
            .map(|PackageDocument { metadata, .. }| metadata.name.as_str())
            .collect();

        loop {
            let used: Vec<&str> = resolved
                .members()
                .filter(|member| {
                    test_crates.contains(member.metadata.name.as_str())
                        && !production_dependents(resolved, &test_crates, member).is_empty()
                })
                .map(|PackageDocument { metadata, .. }| metadata.name.as_str())
                .collect();

            if used.is_empty() {
                return test_crates;
            }

            for name in used {
                test_crates.remove(name);
            }
        }
    }
}

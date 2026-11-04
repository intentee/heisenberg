use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_graph_cycles::cycle_components::CycleComponents;
use heisenberg_graph_cycles::graph_edge::GraphEdge;
use heisenberg_graph_cycles::shortest_cycle::shortest_cycle;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;

use crate::dependency_document::DependencyDocument;
use crate::member_target::MemberTarget;
use crate::package_document::PackageDocument;
use crate::package_violation::PackageViolation;
use crate::potential_owner::PotentialOwner;
use crate::resolved_packages::ResolvedPackages;

pub struct PackageRules<'rules> {
    pub empty_libraries: &'rules BTreeSet<String>,
    pub resolved: &'rules ResolvedPackages,
    pub test_library_packages: &'rules BTreeSet<String>,
}

impl PackageRules<'_> {
    #[must_use]
    pub fn violations(&self) -> Vec<PackageViolation> {
        let mut violations: BTreeSet<PackageViolation> =
            self.dependency_cycles().into_iter().collect();

        violations.extend(self.tests_without_test_libraries());

        violations.into_iter().collect()
    }

    fn blocking_cycle(
        &self,
        destination: &str,
        dependencies: &BTreeSet<&str>,
    ) -> Option<Vec<String>> {
        shortest_cycle(
            self.package_edges().chain(
                dependencies
                    .iter()
                    .filter(|dependency| **dependency != destination)
                    .map(|dependency| GraphEdge {
                        from: destination,
                        to: *dependency,
                    }),
            ),
            destination,
        )
        .map(|cycle| {
            cycle
                .into_iter()
                .map(|package| self.versioned_name(package))
                .collect()
        })
    }

    fn dependency_cycles(&self) -> Vec<PackageViolation> {
        let workspace_path = self.resolved.workspace_path();
        let cycles = CycleComponents::new(self.package_edges());
        let component_of = cycles.component_of();
        let members_by_name: BTreeMap<&str, &PackageDocument> = self
            .resolved
            .members()
            .map(|member| (member.metadata.name.as_str(), member))
            .collect();
        let mut first_members: BTreeMap<&BTreeSet<&str>, &PackageDocument> = BTreeMap::new();

        for member in members_by_name.values() {
            if let Some(component) = component_of.get(member.metadata.id.repr.as_str()) {
                first_members.entry(*component).or_insert(*member);
            }
        }

        first_members
            .into_iter()
            .map(|(component, member)| {
                let manifest = member.relative_manifest(&workspace_path);
                let mut packages: Vec<String> = self
                    .resolved
                    .packages
                    .iter()
                    .filter(|PackageDocument { metadata, .. }| {
                        component.contains(metadata.id.repr.as_str())
                    })
                    .map(PackageDocument::versioned_name)
                    .collect();

                packages.sort();

                match <[String; 1]>::try_from(packages) {
                    Ok([package]) => PackageViolation::PackageDependsOnItself { manifest, package },
                    Err(packages) => {
                        PackageViolation::PackageDependencyCycle { manifest, packages }
                    }
                }
            })
            .collect()
    }

    fn is_tests_package(&self, PackageDocument { metadata, .. }: &PackageDocument) -> bool {
        let name = metadata.name.as_str();
        let targets: Vec<&MemberTarget> = self
            .resolved
            .member_targets
            .iter()
            .filter(|MemberTarget { crate_target, .. }| crate_target.package_name == name)
            .collect();
        let others: Vec<&&MemberTarget> = targets
            .iter()
            .filter(|MemberTarget { crate_target, .. }| {
                crate_target.kind != CrateTargetKind::IntegrationTest
            })
            .collect();

        others.len() < targets.len()
            && others.iter().all(|MemberTarget { crate_target, .. }| {
                crate_target.kind == CrateTargetKind::Library && self.empty_libraries.contains(name)
            })
    }

    fn package_edges(&self) -> impl Iterator<Item = GraphEdge<&str>> {
        self.resolved.packages.iter().flat_map(
            |PackageDocument {
                 dependencies,
                 metadata,
                 ..
             }| {
                dependencies
                    .iter()
                    .map(|DependencyDocument { package_id, .. }| GraphEdge {
                        from: metadata.id.repr.as_str(),
                        to: package_id.as_str(),
                    })
            },
        )
    }

    fn tests_without_test_libraries(&self) -> Vec<PackageViolation> {
        let workspace_path = self.resolved.workspace_path();
        let members: Vec<&PackageDocument> = self.resolved.members().collect();

        members
            .iter()
            .filter(|member| {
                self.is_tests_package(member)
                    && !self
                        .test_library_packages
                        .contains(member.metadata.name.as_str())
            })
            .map(|member| {
                let dependencies: BTreeSet<&str> = member
                    .dependencies
                    .iter()
                    .map(|DependencyDocument { package_id, .. }| package_id.as_str())
                    .collect();
                let potential_owners: BTreeMap<&str, PotentialOwner> = members
                    .iter()
                    .filter(|PackageDocument { metadata, .. }| {
                        dependencies.contains(metadata.id.repr.as_str())
                    })
                    .map(|PackageDocument { metadata, .. }| {
                        (
                            metadata.name.as_str(),
                            self.blocking_cycle(metadata.id.repr.as_str(), &dependencies)
                                .map_or_else(
                                    || PotentialOwner::Free {
                                        package: metadata.name.to_string(),
                                    },
                                    |cycle| PotentialOwner::Blocked {
                                        cycle,
                                        package: metadata.name.to_string(),
                                    },
                                ),
                        )
                    })
                    .collect();

                PackageViolation::TestsOnlyPackageWithoutTestLibraries {
                    manifest: member.relative_manifest(&workspace_path),
                    package: member.metadata.name.to_string(),
                    potential_owners: potential_owners.into_values().collect(),
                }
            })
            .collect()
    }

    fn versioned_name(&self, id: &str) -> String {
        self.resolved
            .packages
            .iter()
            .map(|package| (package.metadata.id.repr.as_str(), package))
            .collect::<BTreeMap<&str, &PackageDocument>>()[id]
            .versioned_name()
    }
}

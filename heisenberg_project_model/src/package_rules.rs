use std::collections::BTreeMap;
use std::collections::BTreeSet;

use cargo::workspace::dependency::DepKind;

use heisenberg_graph_cycles::cycle_components::CycleComponents;
use heisenberg_graph_cycles::graph_edge::GraphEdge;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;

use crate::member_target::MemberTarget;
use crate::package_document::PackageDocument;
use crate::package_violation::PackageViolation;
use crate::resolved_packages::ResolvedPackages;

pub struct PackageRules<'rules> {
    pub empty_libraries: &'rules BTreeSet<String>,
    pub resolved: &'rules ResolvedPackages,
}

impl PackageRules<'_> {
    #[must_use]
    pub fn violations(&self) -> Vec<PackageViolation> {
        let mut violations: BTreeSet<PackageViolation> =
            self.dependency_cycles().into_iter().collect();

        violations.extend(self.tests_without_test_libraries());

        violations.into_iter().collect()
    }

    fn dependency_cycles(&self) -> Vec<PackageViolation> {
        let workspace_path = self.resolved.workspace_path();
        let cycles = CycleComponents::new(self.resolved.packages.iter().flat_map(|package| {
            package.dependencies.iter().map(|dependency| GraphEdge {
                from: package.metadata.id.repr.as_str(),
                to: dependency.package_id.as_str(),
            })
        }));
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
                    .filter(|package| component.contains(package.metadata.id.repr.as_str()))
                    .map(|package| {
                        format!("{} v{}", package.metadata.name, package.metadata.version)
                    })
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

    fn is_tests_only_with_an_empty_library(
        &self,
        PackageDocument { metadata, .. }: &PackageDocument,
    ) -> bool {
        let name = metadata.name.as_str();
        let targets: Vec<&MemberTarget> = self
            .resolved
            .member_targets
            .iter()
            .filter(|target| target.crate_target.package_name == name)
            .collect();
        let others: Vec<&&MemberTarget> = targets
            .iter()
            .filter(|target| target.crate_target.kind != CrateTargetKind::IntegrationTest)
            .collect();

        others.len() < targets.len()
            && matches!(others.as_slice(), [library] if library.crate_target.kind == CrateTargetKind::Library)
            && self.empty_libraries.contains(name)
    }

    fn tests_without_test_libraries(&self) -> Vec<PackageViolation> {
        let workspace_path = self.resolved.workspace_path();
        let members: Vec<&PackageDocument> = self.resolved.members().collect();

        members
            .iter()
            .filter(|member| self.is_tests_only_with_an_empty_library(member))
            .filter_map(|member| {
                let dependencies: BTreeSet<&str> = member
                    .dependencies
                    .iter()
                    .map(|dependency| dependency.package_id.as_str())
                    .collect();
                let destinations: BTreeMap<&str, String> = members
                    .iter()
                    .filter(|candidate| dependencies.contains(candidate.metadata.id.repr.as_str()))
                    .filter(|candidate| {
                        let available: BTreeSet<&str> = candidate
                            .dependencies
                            .iter()
                            .filter(|dependency| {
                                dependency.has_kind(DepKind::Normal)
                                    || dependency.has_kind(DepKind::Development)
                            })
                            .map(|dependency| dependency.package_id.as_str())
                            .collect();

                        dependencies
                            .iter()
                            .filter(|dependency| {
                                **dependency != candidate.metadata.id.repr.as_str()
                            })
                            .all(|dependency| available.contains(dependency))
                    })
                    .map(|candidate| {
                        (
                            candidate.metadata.id.repr.as_str(),
                            candidate.metadata.name.to_string(),
                        )
                    })
                    .collect();

                (!destinations.is_empty()).then(|| {
                    PackageViolation::TestsOnlyPackageWithoutTestLibraries {
                        destinations: destinations.into_values().collect(),
                        manifest: member.relative_manifest(&workspace_path),
                        package: member.metadata.name.to_string(),
                    }
                })
            })
            .collect()
    }
}

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_graph_cycles::cycle_components::CycleComponents;
use heisenberg_graph_cycles::graph_edge::GraphEdge;
use heisenberg_graph_cycles::shortest_paths::ShortestPaths;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;

use crate::benchmark_target_violation::BenchmarkTargetViolation;
use crate::dependency_document::DependencyDocument;
use crate::integration_test_violation::IntegrationTestViolation;
use crate::libraryless_contents::LibrarylessContents;
use crate::libraryless_package::LibrarylessPackage;
use crate::member_target::MemberTarget;
use crate::package_composition::PackageComposition;
use crate::package_document::PackageDocument;
use crate::package_violation::PackageViolation;
use crate::potential_owners::PotentialOwners;
use crate::resolved_packages::ResolvedPackages;
use crate::target_manifest_flags::TargetManifestFlags;

pub struct PackageRules<'rules> {
    pub empty_libraries: &'rules BTreeSet<String>,
    pub helper_holding_packages: &'rules BTreeSet<String>,
    pub resolved: &'rules ResolvedPackages,
}

impl PackageRules<'_> {
    #[must_use]
    pub fn violations(&self) -> Vec<PackageViolation> {
        let mut violations: BTreeSet<PackageViolation> =
            self.dependency_cycles().into_iter().collect();

        violations.extend(self.benchmark_target_flags());
        violations.extend(self.integration_test_targets());
        violations.extend(self.packages_without_libraries());

        violations.into_iter().collect()
    }

    fn benchmark_target_flags(&self) -> Vec<PackageViolation> {
        let workspace_path = self.resolved.workspace_path();

        self.resolved
            .member_targets
            .iter()
            .filter(|MemberTarget { crate_target, .. }| {
                crate_target.kind == CrateTargetKind::Benchmark
            })
            .filter_map(
                |MemberTarget {
                     crate_target,
                     manifest_flags:
                         TargetManifestFlags {
                             benched,
                             harness,
                             tested,
                         },
                     ..
                 }| {
                    self.resolved
                        .member_named(&crate_target.package_name)
                        .map(|member| {
                            let manifest = member.relative_manifest(&workspace_path);
                            let target = crate_target.target_description.clone();

                            [
                                (!benched).then(|| {
                                    BenchmarkTargetViolation::ExcludedFromBenchmarks {
                                        manifest: manifest.clone(),
                                        target: target.clone(),
                                    }
                                }),
                                tested.then(|| BenchmarkTargetViolation::RunByTests {
                                    manifest: manifest.clone(),
                                    target: target.clone(),
                                }),
                                harness.then(|| BenchmarkTargetViolation::WithLibtestHarness {
                                    manifest,
                                    target,
                                }),
                            ]
                        })
                },
            )
            .flatten()
            .flatten()
            .map(PackageViolation::BenchmarkTarget)
            .collect()
    }

    fn composition(
        &self,
        PackageDocument { metadata, .. }: &PackageDocument,
    ) -> PackageComposition {
        let name = metadata.name.as_str();
        let targets: Vec<&MemberTarget> = self
            .resolved
            .member_targets
            .iter()
            .filter(|MemberTarget { crate_target, .. }| crate_target.package_name == name)
            .collect();
        let has_kind = |kind: CrateTargetKind| {
            targets
                .iter()
                .any(|MemberTarget { crate_target, .. }| crate_target.kind == kind)
        };
        let holds_no_library = targets.iter().all(|MemberTarget { crate_target, .. }| {
            crate_target.kind == CrateTargetKind::Benchmark
                || crate_target.kind == CrateTargetKind::IntegrationTest
                || (crate_target.kind == CrateTargetKind::Library
                    && self.empty_libraries.contains(name))
        });

        if !holds_no_library {
            PackageComposition::HoldsProductionCode
        } else if has_kind(CrateTargetKind::IntegrationTest) {
            PackageComposition::WithoutLibrary(LibrarylessContents::Tests)
        } else if has_kind(CrateTargetKind::Benchmark) {
            PackageComposition::WithoutLibrary(LibrarylessContents::Benchmarks)
        } else {
            PackageComposition::HoldsProductionCode
        }
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

    fn integration_test_targets(&self) -> Vec<PackageViolation> {
        let workspace_path = self.resolved.workspace_path();

        self.resolved
            .members()
            .flat_map(|member| {
                let manifest = member.relative_manifest(&workspace_path);
                let name = member.metadata.name.to_string();
                let targets: Vec<String> = self
                    .resolved
                    .member_targets
                    .iter()
                    .filter(
                        |MemberTarget {
                             crate_target,
                             manifest_flags: TargetManifestFlags { harness, .. },
                             ..
                         }| {
                            *harness
                                && crate_target.kind == CrateTargetKind::IntegrationTest
                                && crate_target.package_name == name
                        },
                    )
                    .map(|MemberTarget { crate_target, .. }| {
                        crate_target.target_description.clone()
                    })
                    .collect();

                [
                    (!targets.is_empty() && self.resolved.test_discovering_members.contains(&name))
                        .then(|| IntegrationTestViolation::Discovered {
                            manifest: manifest.clone(),
                        }),
                    (targets.len() > 1)
                        .then_some(IntegrationTestViolation::Split { manifest, targets }),
                ]
            })
            .flatten()
            .map(PackageViolation::IntegrationTests)
            .collect()
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

    fn packages_without_libraries(&self) -> Vec<PackageViolation> {
        let workspace_path = self.resolved.workspace_path();
        let members: Vec<&PackageDocument> = self.resolved.members().collect();
        let paths = ShortestPaths::new(self.package_edges());
        let packages_by_id: BTreeMap<&str, &PackageDocument> = self
            .resolved
            .packages
            .iter()
            .map(|package| (package.metadata.id.repr.as_str(), package))
            .collect();
        let potential_owners = |member: &PackageDocument| {
            PotentialOwners {
                members: &members,
                packages_by_id: &packages_by_id,
                paths: &paths,
            }
            .of(member)
        };

        members
            .iter()
            .filter(|PackageDocument { metadata, .. }| {
                !self
                    .helper_holding_packages
                    .contains(metadata.name.as_str())
            })
            .filter_map(|member| {
                let manifest = member.relative_manifest(&workspace_path);
                let package = member.metadata.name.to_string();

                match self.composition(member) {
                    PackageComposition::HoldsProductionCode => None,
                    PackageComposition::WithoutLibrary(contents) => {
                        Some(PackageViolation::WithoutLibrary(LibrarylessPackage {
                            contents,
                            manifest,
                            package,
                            potential_owners: potential_owners(member),
                        }))
                    }
                }
            })
            .collect()
    }
}

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use cargo_metadata::Source;
use cargo_metadata::Target;
use cargo_metadata::TargetKind;
use ra_ap_paths::AbsPathBuf;
use ra_ap_vfs::VfsPath;

use heisenberg_module_graph::crate_target::CrateTarget;

use crate::confinement_model::ConfinementModel;
use crate::constants::BENCHMARK_HARNESS_PACKAGE;
use crate::constants::ERROR_DERIVE_PACKAGE;
use crate::constants::ERROR_RUNTIME_PACKAGE;
use crate::crate_package::CratePackage;
use crate::declared_confined_packages::DeclaredConfinedPackages;
use crate::declared_test_crates::DeclaredTestCrates;
use crate::member_target::MemberTarget;
use crate::package_document::PackageDocument;
use crate::resolved_packages::ResolvedPackages;

fn vetted_roots(resolved: &ResolvedPackages, name: &str, kind: &TargetKind) -> HashSet<VfsPath> {
    resolved
        .named(name)
        .filter(|PackageDocument { metadata, .. }| {
            metadata.source.as_ref().is_some_and(Source::is_crates_io)
        })
        .flat_map(|PackageDocument { metadata, .. }| &metadata.targets)
        .filter(|Target { kind: kinds, .. }| kinds.contains(kind))
        .map(|Target { src_path, .. }| VfsPath::from(AbsPathBuf::assert(src_path.clone())))
        .collect()
}

pub struct WorkspacePackages {
    pub benchmark_harness_roots: HashSet<VfsPath>,
    pub confinement: ConfinementModel,
    pub crate_packages: HashMap<VfsPath, CratePackage>,
    pub error_derive_roots: HashSet<VfsPath>,
    pub error_runtime_roots: HashSet<VfsPath>,
    pub member_targets: HashMap<VfsPath, CrateTarget>,
    pub test_crates: BTreeSet<String>,
    pub member_directories: Vec<AbsPathBuf>,
    pub unpublished_packages: BTreeSet<String>,
}

impl WorkspacePackages {
    #[must_use]
    pub fn new(
        resolved: &ResolvedPackages,
        declared_confined_packages: &DeclaredConfinedPackages,
        declared_test_crates: &DeclaredTestCrates,
    ) -> Self {
        Self {
            benchmark_harness_roots: vetted_roots(
                resolved,
                BENCHMARK_HARNESS_PACKAGE,
                &TargetKind::Lib,
            ),
            confinement: declared_confined_packages.model(resolved),
            crate_packages: resolved
                .packages
                .iter()
                .flat_map(|PackageDocument { metadata, .. }| {
                    metadata
                        .targets
                        .iter()
                        .filter(|Target { kind, .. }| !kind.contains(&TargetKind::CustomBuild))
                        .map(|Target { src_path, .. }| {
                            (
                                VfsPath::from(AbsPathBuf::assert(src_path.clone())),
                                CratePackage {
                                    id: metadata.id.repr.clone(),
                                    name: metadata.name.to_string(),
                                },
                            )
                        })
                })
                .collect(),
            error_derive_roots: vetted_roots(
                resolved,
                ERROR_DERIVE_PACKAGE,
                &TargetKind::ProcMacro,
            ),
            error_runtime_roots: vetted_roots(resolved, ERROR_RUNTIME_PACKAGE, &TargetKind::Lib),
            member_targets: resolved
                .member_targets
                .iter()
                .map(
                    |MemberTarget {
                         root, crate_target, ..
                     }| { (VfsPath::from(root.clone()), crate_target.clone()) },
                )
                .collect(),
            test_crates: declared_test_crates.test_crates(resolved),
            member_directories: resolved
                .members()
                .filter_map(|PackageDocument { metadata, .. }| metadata.manifest_path.parent())
                .map(|directory| AbsPathBuf::assert(directory.to_path_buf()))
                .collect(),
            unpublished_packages: resolved
                .members()
                .filter(|member| member.is_unpublished())
                .map(|PackageDocument { metadata, .. }| metadata.name.to_string())
                .collect(),
        }
    }
}

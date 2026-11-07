use std::collections::BTreeSet;
use std::collections::HashMap;

use cargo_metadata::Target;
use cargo_metadata::TargetKind;
use ra_ap_paths::AbsPathBuf;
use ra_ap_vfs::VfsPath;

use heisenberg_module_graph::crate_target::CrateTarget;

use crate::declared_test_crates::DeclaredTestCrates;
use crate::member_target::MemberTarget;
use crate::package_document::PackageDocument;
use crate::resolved_packages::ResolvedPackages;

pub struct WorkspacePackages {
    pub crate_packages: HashMap<VfsPath, String>,
    pub member_targets: HashMap<VfsPath, CrateTarget>,
    pub test_crates: BTreeSet<String>,
    pub unpublished_packages: BTreeSet<String>,
}

impl WorkspacePackages {
    #[must_use]
    pub fn new(resolved: &ResolvedPackages, declared_test_crates: &DeclaredTestCrates) -> Self {
        Self {
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
                                metadata.id.repr.clone(),
                            )
                        })
                })
                .collect(),
            member_targets: resolved
                .member_targets
                .iter()
                .map(|MemberTarget { root, crate_target }| {
                    (VfsPath::from(root.clone()), crate_target.clone())
                })
                .collect(),
            test_crates: declared_test_crates.test_crates(resolved),
            unpublished_packages: resolved
                .members()
                .filter(|member| member.is_unpublished())
                .map(|PackageDocument { metadata, .. }| metadata.name.to_string())
                .collect(),
        }
    }
}

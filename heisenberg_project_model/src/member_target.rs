use ra_ap_paths::AbsPathBuf;

use heisenberg_module_graph::crate_target::CrateTarget;

use crate::target_manifest_flags::TargetManifestFlags;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MemberTarget {
    pub crate_target: CrateTarget,
    pub manifest_flags: TargetManifestFlags,
    pub root: AbsPathBuf,
}

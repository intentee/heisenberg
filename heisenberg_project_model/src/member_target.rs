use ra_ap_paths::AbsPathBuf;

use heisenberg_module_graph::crate_target::CrateTarget;

use crate::doc_tests::DocTests;
use crate::target_manifest_flags::TargetManifestFlags;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MemberTarget {
    pub crate_target: CrateTarget,
    pub doc_tests: DocTests,
    pub manifest_flags: TargetManifestFlags,
    pub root: AbsPathBuf,
}

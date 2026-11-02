use ra_ap_paths::AbsPathBuf;

use heisenberg_module_graph::crate_target::CrateTarget;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MemberTarget {
    pub crate_target: CrateTarget,
    pub root: AbsPathBuf,
}

use std::collections::BTreeSet;

use ra_ap_hir::Crate;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::crate_target::CrateTarget;

#[derive(Clone)]
pub struct ResolvedMemberCrate {
    pub crate_target: CrateTarget,
    pub files: BTreeSet<FileId>,
    pub krate: Crate,
}

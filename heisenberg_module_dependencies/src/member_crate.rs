use ra_ap_hir::Crate;

use heisenberg_module_graph::crate_target::CrateTarget;

#[derive(Clone)]
pub struct MemberCrate {
    pub crate_target: CrateTarget,
    pub krate: Crate,
}

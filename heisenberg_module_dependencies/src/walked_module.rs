use heisenberg_module_graph::module_findings::ModuleFindings;

use crate::crate_findings::CrateFindings;

#[derive(Default)]
pub struct WalkedModule {
    pub crate_findings: CrateFindings,
    pub findings: ModuleFindings,
}

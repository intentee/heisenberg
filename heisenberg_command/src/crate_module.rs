use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::module_path::ModulePath;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CrateModule {
    pub module: ModulePath,
    pub crate_target: CrateTarget,
}

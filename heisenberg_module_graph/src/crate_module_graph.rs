use std::collections::BTreeSet;

use crate::crate_target::CrateTarget;
use crate::function_body::FunctionBody;
use crate::module_findings::ModuleFindings;
use crate::module_path::ModulePath;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CrateModuleGraph {
    pub crate_target: CrateTarget,
    pub findings: ModuleFindings,
    pub function_bodies: BTreeSet<FunctionBody>,
    pub modules: BTreeSet<ModulePath>,
    pub root_module_is_empty: bool,
}

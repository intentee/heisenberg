use std::collections::BTreeSet;

use crate::crate_target::CrateTarget;
use crate::function_body::FunctionBody;
use crate::module_findings::ModuleFindings;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CrateModuleGraph {
    pub crate_target: CrateTarget,
    pub findings: ModuleFindings,
    pub function_bodies: BTreeSet<FunctionBody>,
    pub module_count: usize,
    pub module_files: BTreeSet<String>,
    pub root_module_is_empty: bool,
    pub runs_libtest_tests: bool,
}

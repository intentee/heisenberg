use std::collections::BTreeSet;

use crate::configured_out_negations::ConfiguredOutNegations;
use crate::crate_target::CrateTarget;
use crate::function_body::FunctionBody;
use crate::module_dependency::ModuleDependency;
use crate::module_path::ModulePath;
use crate::shared_module_file::SharedModuleFile;
use crate::violation_set::ViolationSet;
use crate::walked_code::WalkedCode;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CrateModuleGraph {
    pub configured_out_negations: ConfiguredOutNegations,
    pub crate_target: CrateTarget,
    pub dependencies: BTreeSet<ModuleDependency>,
    pub function_bodies: BTreeSet<FunctionBody>,
    pub modules: BTreeSet<ModulePath>,
    pub root_module_is_empty: bool,
    pub shared_module_files: BTreeSet<SharedModuleFile>,
    pub violations: ViolationSet,
    pub walked_code: WalkedCode,
}

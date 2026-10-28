use std::collections::BTreeSet;

use crate::crate_target::CrateTarget;
use crate::module_dependency::ModuleDependency;
use crate::module_path::ModulePath;
use crate::violation::Violation;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CrateModuleGraph {
    pub crate_target: CrateTarget,
    pub dependencies: BTreeSet<ModuleDependency>,
    pub modules: BTreeSet<ModulePath>,
    pub violations: BTreeSet<Violation>,
}

use std::collections::BTreeSet;

use crate::crate_target::CrateTarget;
use crate::inactive_region::InactiveRegion;
use crate::module_dependency::ModuleDependency;
use crate::module_path::ModulePath;
use crate::shared_module_file::SharedModuleFile;
use crate::violation::Violation;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CrateModuleGraph {
    pub crate_target: CrateTarget,
    pub dependencies: BTreeSet<ModuleDependency>,
    pub inactive_regions: BTreeSet<InactiveRegion>,
    pub modules: BTreeSet<ModulePath>,
    pub shared_module_files: BTreeSet<SharedModuleFile>,
    pub violations: BTreeSet<Violation>,
    pub walked_files: BTreeSet<String>,
}

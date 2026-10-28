use heisenberg_module_graph::module_dependency::ModuleDependency;

use crate::module_unit::ModuleUnit;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CycleWitness {
    pub dependency: ModuleDependency,
    pub from: ModuleUnit,
    pub to: ModuleUnit,
}

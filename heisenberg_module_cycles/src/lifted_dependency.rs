use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_path::ModulePath;

use crate::module_unit::ModuleUnit;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LiftedDependency {
    pub container: ModulePath,
    pub from: ModuleUnit,
    pub to: ModuleUnit,
}

impl LiftedDependency {
    #[must_use]
    pub fn of(ModuleDependency { from, to, .. }: &ModuleDependency) -> Self {
        let container = from.common_ancestor(to);

        Self {
            from: ModuleUnit::within(&container, from),
            to: ModuleUnit::within(&container, to),
            container,
        }
    }
}

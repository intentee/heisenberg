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
    pub fn of(dependency: &ModuleDependency) -> Self {
        let container = dependency.from.common_ancestor(&dependency.to);

        Self {
            from: ModuleUnit::within(&container, &dependency.from),
            to: ModuleUnit::within(&container, &dependency.to),
            container,
        }
    }
}

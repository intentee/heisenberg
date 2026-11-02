use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_path::ModulePath;

use crate::module_unit::ModuleUnit;
use crate::sibling_edge::SiblingEdge;

pub struct LiftedDependency {
    pub container: ModulePath,
    pub edge: SiblingEdge,
}

impl LiftedDependency {
    #[must_use]
    pub fn of(ModuleDependency { from, to, .. }: &ModuleDependency) -> Self {
        let container = from.common_ancestor(to);

        Self {
            edge: SiblingEdge {
                from: ModuleUnit::within(&container, from),
                to: ModuleUnit::within(&container, to),
            },
            container,
        }
    }
}

use heisenberg_module_graph::module_dependency::ModuleDependency;

use crate::non_empty::NonEmpty;
use crate::sibling_edge::SiblingEdge;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CycleWitness {
    pub dependencies: NonEmpty<ModuleDependency>,
    pub edge: SiblingEdge,
}

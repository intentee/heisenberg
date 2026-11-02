use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::module_path::ModulePath;

use crate::cycle_witness::CycleWitness;
use crate::non_empty::NonEmpty;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModuleCycle {
    pub container: ModulePath,
    pub crate_target: CrateTarget,
    pub witnesses: NonEmpty<CycleWitness>,
}

use heisenberg_module_cycles::cycle_witness::CycleWitness;
use heisenberg_module_cycles::non_empty::NonEmpty;
use heisenberg_module_graph::module_path::ModulePath;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CycleShape<'cycle> {
    pub container: &'cycle ModulePath,
    pub witnesses: &'cycle NonEmpty<CycleWitness>,
}

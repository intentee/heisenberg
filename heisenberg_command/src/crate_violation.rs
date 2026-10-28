use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::violation::Violation;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CrateViolation {
    pub crate_target: CrateTarget,
    pub violation: Violation,
}

use heisenberg_module_cycles::module_cycle::ModuleCycle;

use crate::crate_violation::CrateViolation;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Finding {
    Cycle(ModuleCycle),
    Violation(CrateViolation),
}

use ra_ap_hir::Adt;

use heisenberg_module_graph::violation::Violation;

pub struct AdtViolation {
    pub adt: Adt,
    pub violation: Violation,
}

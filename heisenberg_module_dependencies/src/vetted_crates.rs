use std::collections::HashSet;

use ra_ap_hir::Crate;

#[derive(Default)]
pub struct VettedCrates {
    pub benchmark_harness: HashSet<Crate>,
    pub error_derive: HashSet<Crate>,
    pub error_runtime: HashSet<Crate>,
}

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;

use crate::test_target_findings::TestTargetFindings;
use crate::unsafe_code_findings::UnsafeCodeFindings;

#[derive(Default)]
pub struct CrateFindings {
    pub test_target: TestTargetFindings,
    pub unsafe_code: UnsafeCodeFindings,
}

impl CrateFindings {
    pub fn fold(self, graph: &mut CrateModuleGraph) {
        let Self {
            test_target,
            unsafe_code,
        } = self;

        test_target.fold(graph);
        unsafe_code.fold(graph);
    }
}

use heisenberg_module_graph::module_findings::ModuleFindings;

use crate::test_target_findings::TestTargetFindings;
use crate::unsafe_code_findings::UnsafeCodeFindings;

#[derive(Default)]
pub struct CrateFindings {
    pub test_target: TestTargetFindings,
    pub unsafe_code: UnsafeCodeFindings,
}

impl CrateFindings {
    pub fn fold(self, module_findings: &mut ModuleFindings) {
        let Self {
            test_target,
            unsafe_code,
        } = self;

        test_target.fold(module_findings);
        unsafe_code.fold(module_findings);
    }

    pub fn merge(
        &mut self,
        Self {
            test_target,
            unsafe_code,
        }: Self,
    ) {
        self.test_target.merge(test_target);
        self.unsafe_code.merge(unsafe_code);
    }
}

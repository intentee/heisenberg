use heisenberg_module_graph::module_findings::ModuleFindings;

use crate::error_type_findings::ErrorTypeFindings;
use crate::test_target_findings::TestTargetFindings;
use crate::unsafe_code_findings::UnsafeCodeFindings;

#[derive(Default)]
pub struct CrateFindings {
    pub error_types: ErrorTypeFindings,
    pub registers_benchmark: bool,
    pub runs_libtest_tests: bool,
    pub test_target: TestTargetFindings,
    pub unsafe_code: UnsafeCodeFindings,
}

impl CrateFindings {
    pub fn fold(self, module_findings: &mut ModuleFindings) {
        let Self {
            error_types,
            test_target,
            unsafe_code,
            ..
        } = self;

        error_types.fold(module_findings);
        test_target.fold(module_findings);
        unsafe_code.fold(module_findings);
    }

    pub fn merge(
        &mut self,
        Self {
            error_types,
            registers_benchmark,
            runs_libtest_tests,
            test_target,
            unsafe_code,
        }: Self,
    ) {
        self.error_types.merge(error_types);
        self.registers_benchmark |= registers_benchmark;
        self.runs_libtest_tests |= runs_libtest_tests;
        self.test_target.merge(test_target);
        self.unsafe_code.merge(unsafe_code);
    }
}

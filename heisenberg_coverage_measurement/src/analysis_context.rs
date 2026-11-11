use heisenberg_configured_code::test_module_code::TestModuleCode;
use heisenberg_source_locator::source_locator::SourceLocator;

use crate::active_code::ActiveCode;
use crate::function_identity::FunctionIdentity;

pub struct AnalysisContext<'context, 'database> {
    pub active_code: &'context ActiveCode,
    pub identity: &'context FunctionIdentity<'context, 'database>,
    pub locator: &'context SourceLocator<'context>,
    pub test_modules: &'context TestModuleCode<'context, 'database>,
}

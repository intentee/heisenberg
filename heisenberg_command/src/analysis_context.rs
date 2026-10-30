use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_project_model::supported_target::SupportedTarget;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AnalysisContext {
    pub crate_target: CrateTarget,
    pub target: SupportedTarget,
}

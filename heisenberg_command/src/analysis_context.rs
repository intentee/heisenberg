use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_project_model::configuration::Configuration;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AnalysisContext {
    pub configuration: Configuration,
    pub crate_target: CrateTarget,
}

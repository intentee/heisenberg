use heisenberg_module_cycles::module_cycle::ModuleCycle;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_project_model::supported_target::SupportedTarget;

pub struct TargetAnalysis {
    pub cycles: Vec<ModuleCycle>,
    pub graphs: Vec<CrateModuleGraph>,
    pub target: SupportedTarget,
}

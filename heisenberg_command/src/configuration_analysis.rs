use std::collections::BTreeSet;

use heisenberg_module_cycles::module_cycle::ModuleCycle;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::confined_package_reach::ConfinedPackageReach;

pub struct ConfigurationAnalysis {
    pub configuration: Configuration,
    pub confinement: ConfinedPackageReach,
    pub cycles: Vec<ModuleCycle>,
    pub graphs: Vec<CrateModuleGraph>,
    pub test_directory_files: BTreeSet<String>,
}

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_module_cycles::module_cycle::ModuleCycle;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::confined_package_reach::ConfinedPackageReach;

pub struct ConfigurationAnalysis {
    pub build_script_output_files: BTreeSet<String>,
    pub build_script_variables: BTreeMap<String, BTreeSet<String>>,
    pub configuration: Configuration,
    pub confinement: ConfinedPackageReach,
    pub cycles: Vec<ModuleCycle>,
    pub graphs: Vec<CrateModuleGraph>,
    pub member_source_files: BTreeSet<String>,
}

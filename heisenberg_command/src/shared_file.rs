use std::collections::BTreeMap;

use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_project_model::configuration::Configuration;

use crate::module_inclusion::ModuleInclusion;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SharedFile<'files> {
    pub file: &'files str,
    pub inclusions: &'files BTreeMap<ModuleInclusion, BTreeMap<Configuration, ModuleFileAnalysis>>,
}

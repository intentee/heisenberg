use std::collections::BTreeMap;

use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_project_model::supported_target::SupportedTarget;

use crate::module_inclusion::ModuleInclusion;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SharedFile<'files> {
    pub file: &'files str,
    pub inclusions:
        &'files BTreeMap<ModuleInclusion, BTreeMap<SupportedTarget, ModuleFileAnalysis>>,
}

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_project_model::configuration::Configuration;

use crate::analysis_context::AnalysisContext;

#[derive(Default)]
pub struct AnalysisContexts {
    pub configurations: BTreeSet<Configuration>,
    pub files: BTreeMap<String, BTreeSet<AnalysisContext>>,
}

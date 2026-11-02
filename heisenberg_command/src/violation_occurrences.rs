use std::collections::BTreeSet;

use heisenberg_module_graph::folded_finding::FoldedFinding;

use crate::analysis_context::AnalysisContext;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ViolationOccurrences {
    pub folded: BTreeSet<FoldedFinding>,
    pub reported: BTreeSet<AnalysisContext>,
}

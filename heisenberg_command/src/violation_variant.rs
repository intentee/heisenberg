use std::collections::BTreeSet;

use heisenberg_module_graph::violation_kind::ViolationKind;

use crate::analysis_context::AnalysisContext;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ViolationVariant {
    pub kind: ViolationKind,
    pub reported: BTreeSet<AnalysisContext>,
}

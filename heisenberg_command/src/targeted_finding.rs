use std::collections::BTreeSet;

use heisenberg_project_model::supported_target::SupportedTarget;

use crate::finding::Finding;

pub struct TargetedFinding {
    pub finding: Finding,
    pub targets: BTreeSet<SupportedTarget>,
}

use std::collections::BTreeSet;

use crate::feature_selection::FeatureSelection;
use crate::supported_target::SupportedTarget;

#[derive(Clone, Debug)]
pub struct RunScope {
    pub features: FeatureSelection,
    pub left_out_targets: BTreeSet<SupportedTarget>,
}

impl RunScope {
    #[must_use]
    pub fn is_partial(&self) -> bool {
        !self.left_out_targets.is_empty() || self.features != FeatureSelection::All
    }
}

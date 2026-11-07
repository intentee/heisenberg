use crate::feature_selection::FeatureSelection;
use crate::supported_target::SupportedTarget;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Configuration {
    pub name: String,
    pub target: SupportedTarget,
    pub features: FeatureSelection,
}

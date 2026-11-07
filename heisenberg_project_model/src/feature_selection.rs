use std::collections::BTreeSet;
use std::rc::Rc;

use cargo::resolver::features::CliFeatures;
use cargo::util::interning::InternedString;
use cargo::workspace::FeatureValue;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FeatureSelection {
    pub default_features: bool,
    pub features: BTreeSet<String>,
}

impl FeatureSelection {
    #[must_use]
    pub fn cli_features(&self) -> CliFeatures {
        CliFeatures {
            all_features: false,
            features: Rc::new(
                self.features
                    .iter()
                    .map(|feature| FeatureValue::Feature(InternedString::new(feature)))
                    .collect(),
            ),
            uses_default_features: self.default_features,
        }
    }

    #[must_use]
    pub fn is_default(&self) -> bool {
        self.default_features && self.features.is_empty()
    }
}

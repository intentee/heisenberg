use std::num::NonZeroU64;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(from = "NonZeroU64")]
pub enum NextestTermination {
    AfterPeriods {
        periods: NonZeroU64,
    },
    #[default]
    Never,
}

impl From<NonZeroU64> for NextestTermination {
    fn from(periods: NonZeroU64) -> Self {
        Self::AfterPeriods { periods }
    }
}

use serde::Deserialize;

use crate::nextest_period_setting::NextestPeriodSetting;
use crate::nextest_timeout_limits::NextestTimeoutLimits;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum NextestSlowTimeout {
    Limits(NextestTimeoutLimits),
    Text(NextestPeriodSetting),
}

impl Default for NextestSlowTimeout {
    fn default() -> Self {
        Self::Text(NextestPeriodSetting::Inherited)
    }
}

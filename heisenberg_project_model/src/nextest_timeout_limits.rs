use std::collections::BTreeMap;

use serde::Deserialize;
use toml::Value;

use crate::nextest_duration::NextestDuration;
use crate::nextest_termination::NextestTermination;
use crate::nextest_timeout_result::NextestTimeoutResult;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NextestTimeoutLimits {
    #[serde(default)]
    pub on_timeout: NextestTimeoutResult,
    pub period: NextestDuration,
    #[serde(flatten)]
    pub settings: BTreeMap<String, Value>,
    #[serde(default)]
    pub terminate_after: NextestTermination,
}

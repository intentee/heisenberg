use std::collections::BTreeMap;

use serde::Deserialize;
use toml::Value;

use crate::nextest_slow_timeout::NextestSlowTimeout;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NextestProfile {
    #[serde(default)]
    pub overrides: Vec<Value>,
    #[serde(flatten)]
    pub settings: BTreeMap<String, Value>,
    #[serde(default)]
    pub slow_timeout: NextestSlowTimeout,
}

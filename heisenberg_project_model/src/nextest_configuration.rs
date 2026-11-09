use std::collections::BTreeMap;

use serde::Deserialize;
use toml::Value;

use crate::nextest_profile::NextestProfile;

#[derive(Debug, Deserialize)]
pub struct NextestConfiguration {
    #[serde(default)]
    pub profile: BTreeMap<String, NextestProfile>,
    #[serde(flatten)]
    pub settings: BTreeMap<String, Value>,
}

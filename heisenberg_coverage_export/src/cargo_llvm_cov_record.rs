use serde::Deserialize;
use serde::de::IgnoredAny;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CargoLlvmCovRecord {
    pub manifest_path: String,
    pub version: IgnoredAny,
}

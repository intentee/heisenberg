use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredBuildScriptOutput {
    pub cfgs: Vec<String>,
    pub directory: String,
    pub env: BTreeMap<String, String>,
}

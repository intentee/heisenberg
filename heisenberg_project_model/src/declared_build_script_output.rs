use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredBuildScriptOutput {
    pub cfgs: Vec<String>,
    pub env: BTreeMap<String, String>,
}

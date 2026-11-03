use std::collections::BTreeMap;

use serde::Deserialize;

use crate::declared_build_script_output::DeclaredBuildScriptOutput;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredOutDir {
    pub directory: String,
    pub targets: BTreeMap<String, DeclaredBuildScriptOutput>,
}

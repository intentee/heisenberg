use std::collections::BTreeMap;

use serde::Deserialize;

use crate::declared_build_script_output::DeclaredBuildScriptOutput;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredOutDir {
    pub configurations: BTreeMap<String, DeclaredBuildScriptOutput>,
}

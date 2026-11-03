use std::collections::BTreeMap;

use cargo_metadata::BuildScript;
use cargo_metadata::camino::Utf8PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildScriptOutput {
    pub cfgs: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub out_dir: Utf8PathBuf,
}

impl From<&BuildScript> for BuildScriptOutput {
    fn from(executed: &BuildScript) -> Self {
        Self {
            cfgs: executed.cfgs.clone(),
            env: executed.env.iter().cloned().collect(),
            out_dir: executed.out_dir.clone(),
        }
    }
}

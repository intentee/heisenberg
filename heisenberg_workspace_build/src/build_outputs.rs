use std::collections::BTreeMap;

use cargo_metadata::camino::Utf8PathBuf;

use crate::build_script_output::BuildScriptOutput;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BuildOutputs {
    pub build_scripts: BTreeMap<String, BuildScriptOutput>,
    pub proc_macro_dylibs: BTreeMap<Utf8PathBuf, Utf8PathBuf>,
}

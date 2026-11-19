use std::collections::BTreeSet;

use crate::build_script_origin::BuildScriptOrigin;
use crate::configuration::Configuration;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BuildScriptVariables {
    pub configurations: BTreeSet<Configuration>,
    pub origin: BuildScriptOrigin,
    pub package: String,
    pub variables: BTreeSet<String>,
}

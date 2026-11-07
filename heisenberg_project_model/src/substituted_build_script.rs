use std::collections::BTreeMap;

use heisenberg_workspace_build::build_script_output::BuildScriptOutput;

use crate::configuration::Configuration;

pub struct SubstitutedBuildScript {
    pub configurations: BTreeMap<Configuration, BuildScriptOutput>,
}

impl SubstitutedBuildScript {
    pub fn output(&self, configuration: &Configuration) -> BuildScriptOutput {
        self.configurations[configuration].clone()
    }
}

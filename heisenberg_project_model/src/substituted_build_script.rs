use std::collections::BTreeMap;

use ra_ap_paths::AbsPathBuf;

use heisenberg_workspace_build::build_script_output::BuildScriptOutput;

use crate::declared_build_script_output::DeclaredBuildScriptOutput;
use crate::supported_target::SupportedTarget;

pub struct SubstitutedBuildScript {
    pub directory: AbsPathBuf,
    pub targets: BTreeMap<SupportedTarget, DeclaredBuildScriptOutput>,
}

impl SubstitutedBuildScript {
    pub fn output(&self, target: SupportedTarget) -> BuildScriptOutput {
        let DeclaredBuildScriptOutput { cfgs, env } = &self.targets[&target];

        BuildScriptOutput {
            cfgs: cfgs.clone(),
            env: env.clone(),
            out_dir: self.directory.join(target.triple()).into(),
        }
    }
}

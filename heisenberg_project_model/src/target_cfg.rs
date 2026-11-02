use std::sync::Arc;

use guppy::platform::Platform;
use ra_ap_cfg::CfgAtom;

use crate::project_model_error::ProjectModelError;
use crate::rustc_cfg::RustcCfg;
use crate::supported_target::SupportedTarget;

pub struct TargetCfg {
    pub crates: Vec<CfgAtom>,
    pub library: Vec<CfgAtom>,
    pub platform: Arc<Platform>,
}

impl TargetCfg {
    pub fn parse(target: SupportedTarget) -> Result<Self, ProjectModelError> {
        RustcCfg::parse(target.crates_cfg())
            .map_err(ProjectModelError::CratesCfgTable)
            .and_then(|crates| {
                crates.platform(target.triple()).and_then(|platform| {
                    RustcCfg::parse(target.library_cfg())
                        .map_err(ProjectModelError::LibraryCfgTable)
                        .map(|library| Self {
                            crates: crates.atoms,
                            library: library.atoms,
                            platform,
                        })
                })
            })
    }
}

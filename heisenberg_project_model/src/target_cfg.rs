use std::sync::Arc;

use guppy::platform::Platform;
use ra_ap_cfg::CfgAtom;
use serde_json::from_str;

use crate::project_model_error::ProjectModelError;
use crate::rustc_cfg::RustcCfg;
use crate::supported_target::SupportedTarget;
use crate::target_spec::TargetSpec;

pub struct TargetCfg {
    pub crates: Vec<CfgAtom>,
    pub data_layout: Box<str>,
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
                        .and_then(|library| {
                            from_str::<TargetSpec>(target.target_spec())
                                .map_err(ProjectModelError::TargetSpec)
                                .map(|TargetSpec { data_layout }| Self {
                                    crates: crates.atoms,
                                    data_layout: data_layout.into_boxed_str(),
                                    library: library.atoms,
                                    platform,
                                })
                        })
                })
            })
    }
}

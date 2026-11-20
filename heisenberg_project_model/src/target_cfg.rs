use std::sync::Arc;

use guppy::platform::Platform;
use ra_ap_cfg::CfgAtom;

use crate::project_model_error::ProjectModelError;
use crate::rustc_cfg::RustcCfg;
use crate::supported_target::SupportedTarget;

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
                        .map(|RustcCfg { atoms, .. }| Self {
                            crates: crates.atoms,
                            data_layout: target.data_layout().into(),
                            library: atoms,
                            platform,
                        })
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::TargetCfg;
    use crate::constants::SUPPORTED_TARGETS;

    #[test]
    fn every_supported_target_has_tables_that_parse() {
        let failures: Vec<Option<String>> = SUPPORTED_TARGETS
            .into_iter()
            .map(|target| {
                TargetCfg::parse(target)
                    .err()
                    .as_ref()
                    .map(ToString::to_string)
            })
            .collect();

        assert_eq!(failures, vec![None; SUPPORTED_TARGETS.len()]);
    }
}

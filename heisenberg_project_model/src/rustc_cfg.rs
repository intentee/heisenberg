use std::str::FromStr;
use std::sync::Arc;

use cargo_platform::Cfg;
use cargo_platform::ParseError;
use guppy::platform::Platform;
use guppy::platform::TargetFeatures;
use ra_ap_cfg::CfgAtom;
use ra_ap_intern::sym;

use crate::cfg_atom::cfg_atom;
use crate::project_model_error::ProjectModelError;

pub struct RustcCfg {
    pub atoms: Vec<CfgAtom>,
    pub table: &'static str,
}

impl RustcCfg {
    pub fn parse(table: &'static str) -> Result<Self, ParseError> {
        table
            .lines()
            .map(Cfg::from_str)
            .collect::<Result<Vec<Cfg>, ParseError>>()
            .map(|cfgs| Self {
                atoms: cfgs.into_iter().map(cfg_atom).collect(),
                table,
            })
    }

    pub fn platform(&self, triple: &'static str) -> Result<Arc<Platform>, ProjectModelError> {
        Platform::new_custom_cfg(
            triple,
            self.table,
            TargetFeatures::features(self.atoms.iter().filter_map(|atom| match atom {
                CfgAtom::KeyValue { key, value } if *key == sym::target_feature => {
                    Some(value.as_str().to_owned())
                }
                CfgAtom::Flag(_) | CfgAtom::KeyValue { .. } => None,
            })),
        )
        .map_err(ProjectModelError::TargetPlatform)
        .map(|mut platform| {
            platform.add_flags(self.atoms.iter().filter_map(|atom| match atom {
                CfgAtom::Flag(name) => Some(name.as_str().to_owned()),
                CfgAtom::KeyValue { .. } => None,
            }));

            Arc::new(platform)
        })
    }
}

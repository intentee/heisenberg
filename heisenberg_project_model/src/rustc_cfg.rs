use std::str::FromStr;
use std::sync::Arc;

use cargo_platform::Cfg;
use guppy::platform::Platform;
use guppy::platform::TargetFeatures;
use ra_ap_cfg::CfgAtom;
use ra_ap_intern::Symbol;
use ra_ap_intern::sym;

pub struct RustcCfg {
    pub atoms: Vec<CfgAtom>,
    pub table: &'static str,
}

impl RustcCfg {
    #[expect(
        clippy::expect_used,
        reason = "the embedded tables are verbatim `rustc --print cfg` output, which cargo's own parser reads"
    )]
    #[must_use]
    pub fn parse(table: &'static str) -> Self {
        let atoms = table
            .lines()
            .map(Cfg::from_str)
            .collect::<Result<Vec<Cfg>, _>>()
            .expect("cargo parses every line of `rustc --print cfg` output")
            .into_iter()
            .map(|cfg| match cfg {
                Cfg::Name(name) => CfgAtom::Flag(Symbol::intern(&name.name)),
                Cfg::KeyPair(key, value) => CfgAtom::KeyValue {
                    key: Symbol::intern(&key.name),
                    value: Symbol::intern(&value),
                },
            })
            .collect();

        Self { atoms, table }
    }

    #[expect(
        clippy::expect_used,
        reason = "target-spec reads the same embedded `rustc --print cfg` table that cargo's parser accepted"
    )]
    #[must_use]
    pub fn platform(&self, triple: &'static str) -> Arc<Platform> {
        let mut platform = Platform::new_custom_cfg(
            triple,
            self.table,
            TargetFeatures::features(self.atoms.iter().filter_map(|atom| match atom {
                CfgAtom::KeyValue { key, value } if *key == sym::target_feature => {
                    Some(value.as_str().to_owned())
                }
                CfgAtom::Flag(_) | CfgAtom::KeyValue { .. } => None,
            })),
        )
        .expect("target-spec reads `rustc --print cfg` output");

        platform.add_flags(self.atoms.iter().filter_map(|atom| match atom {
            CfgAtom::Flag(name) => Some(name.as_str().to_owned()),
            CfgAtom::KeyValue { .. } => None,
        }));

        Arc::new(platform)
    }
}

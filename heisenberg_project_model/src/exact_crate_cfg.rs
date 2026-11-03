use std::collections::HashMap;

use ra_ap_cfg::CfgAtom;
use ra_ap_cfg::CfgDiff;
use ra_ap_cfg::CfgOptions;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::all_crates;
use ra_ap_ide_db::base_db::salsa::Durability;
use ra_ap_ide_db::base_db::salsa::Setter;
use ra_ap_intern::sym;
use ra_ap_vfs::Vfs;
use ra_ap_vfs::VfsPath;

use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::test_harness::TestHarness;

pub struct ExactCrateCfg<'assignment> {
    pub build_script_atoms: &'assignment HashMap<VfsPath, Vec<CfgAtom>>,
    pub library_atoms: &'assignment [CfgAtom],
    pub member_targets: &'assignment HashMap<VfsPath, CrateTarget>,
    pub vfs: &'assignment Vfs,
}

impl ExactCrateCfg<'_> {
    pub fn assign(&self, database: &mut RootDatabase) {
        for krate in all_crates(database).iter() {
            let data = krate.data(database);
            let current = krate.cfg_options(database);
            let options = if data.origin.is_lang() {
                self.library_atoms
                    .iter()
                    .cloned()
                    .chain(
                        current
                            .get_cfg_values(sym::feature.as_str())
                            .map(|feature| CfgAtom::KeyValue {
                                key: sym::feature,
                                value: feature.clone(),
                            }),
                    )
                    .collect::<CfgOptions>()
            } else {
                let mut options = current.clone();
                let root = self.vfs.file_path(data.root_file_id);

                if let Some(atoms) = self.build_script_atoms.get(root) {
                    options.apply_diff(CfgDiff::new(atoms.clone(), Vec::new()));
                }

                if self.member_targets.get(root).is_some_and(
                    |CrateTarget { test_harness, .. }| *test_harness == TestHarness::Libtest,
                ) {
                    options.insert_atom(sym::test);
                }

                if data.is_proc_macro {
                    options.insert_atom(sym::proc_macro);
                }

                options
            };

            krate
                .set_cfg_options(database)
                .with_durability(Durability::MEDIUM)
                .to(options);
        }
    }
}

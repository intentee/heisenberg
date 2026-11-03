use std::collections::BTreeMap;
use std::collections::HashMap;

use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::all_crates;
use ra_ap_ide_db::base_db::salsa::Durability;
use ra_ap_ide_db::base_db::salsa::Setter;
use ra_ap_paths::AbsPathBuf;
use ra_ap_vfs::Vfs;
use ra_ap_vfs::VfsPath;

use heisenberg_module_graph::crate_target::CrateTarget;

use crate::constants::CARGO_OUT_DIR_VARIABLE;
use crate::supported_target::SupportedTarget;

pub struct OutDirEnvironment<'assignment> {
    pub member_targets: &'assignment HashMap<VfsPath, CrateTarget>,
    pub out_dirs: &'assignment BTreeMap<String, AbsPathBuf>,
    pub target: SupportedTarget,
    pub vfs: &'assignment Vfs,
}

impl OutDirEnvironment<'_> {
    pub fn assign(&self, database: &mut RootDatabase) {
        for krate in all_crates(database).iter() {
            if let Some(directory) = self
                .member_targets
                .get(self.vfs.file_path(krate.data(database).root_file_id))
                .and_then(|CrateTarget { package_name, .. }| self.out_dirs.get(package_name))
            {
                let mut environment = krate.env(database).clone();

                environment.set(
                    CARGO_OUT_DIR_VARIABLE,
                    directory.join(self.target.triple()).to_string(),
                );
                krate
                    .set_env(database)
                    .with_durability(Durability::MEDIUM)
                    .to(environment);
            }
        }
    }
}

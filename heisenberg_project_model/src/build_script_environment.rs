use std::collections::BTreeMap;
use std::collections::HashMap;

use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::all_crates;
use ra_ap_ide_db::base_db::salsa::Durability;
use ra_ap_ide_db::base_db::salsa::Setter;
use ra_ap_vfs::Vfs;
use ra_ap_vfs::VfsPath;

use heisenberg_workspace_build::build_script_output::BuildScriptOutput;

use crate::constants::CARGO_OUT_DIR_VARIABLE;
use crate::crate_package::CratePackage;

pub struct BuildScriptEnvironment<'assignment> {
    pub crate_packages: &'assignment HashMap<VfsPath, CratePackage>,
    pub outputs: &'assignment BTreeMap<String, BuildScriptOutput>,
    pub vfs: &'assignment Vfs,
}

impl BuildScriptEnvironment<'_> {
    pub fn assign(&self, database: &mut RootDatabase) {
        for krate in all_crates(database).iter() {
            if let Some(BuildScriptOutput { env, out_dir, .. }) = self
                .crate_packages
                .get(self.vfs.file_path(krate.data(database).root_file_id))
                .and_then(|CratePackage { id, .. }| self.outputs.get(id))
            {
                let mut environment = krate.env(database).clone();

                environment.set(CARGO_OUT_DIR_VARIABLE, out_dir.to_string());

                for (name, value) in env {
                    environment.set(name, value.clone());
                }

                krate
                    .set_env(database)
                    .with_durability(Durability::MEDIUM)
                    .to(environment);
            }
        }
    }
}

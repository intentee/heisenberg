use std::collections::HashMap;

use ra_ap_ide_db::RootDatabase;
use ra_ap_paths::AbsPathBuf;
use ra_ap_vfs::Vfs;
use ra_ap_vfs::VfsPath;

use heisenberg_module_graph::crate_target::CrateTarget;

pub struct LoadedProject {
    pub database: RootDatabase,
    pub member_targets: HashMap<VfsPath, CrateTarget>,
    pub vfs: Vfs,
    pub workspace_root: AbsPathBuf,
}

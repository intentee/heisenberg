use std::collections::BTreeSet;

use ra_ap_ide_db::RootDatabase;
use ra_ap_paths::AbsPathBuf;
use ra_ap_vfs::FileId;

use crate::source_files::SourceFiles;
use crate::workspace_packages::WorkspacePackages;

pub struct LoadedProject<'resolved> {
    pub build_script_output_files: BTreeSet<FileId>,
    pub database: RootDatabase,
    pub files: SourceFiles,
    pub packages: &'resolved WorkspacePackages,
    pub workspace_root: &'resolved AbsPathBuf,
}

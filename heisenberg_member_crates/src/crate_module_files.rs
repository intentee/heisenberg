use std::collections::BTreeSet;

use ra_ap_hir::Crate;
use ra_ap_ide_db::RootDatabase;
use ra_ap_vfs::FileId;

pub fn crate_module_files(database: &RootDatabase, krate: Crate) -> BTreeSet<FileId> {
    krate
        .modules(database)
        .iter()
        .filter_map(|module| module.as_source_file_id(database))
        .map(|source_file_id| source_file_id.file_id(database))
        .collect()
}

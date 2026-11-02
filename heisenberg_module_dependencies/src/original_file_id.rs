use ra_ap_hir::HirFileId;
use ra_ap_ide_db::RootDatabase;
use ra_ap_vfs::FileId;

pub fn original_file_id(database: &RootDatabase, file_id: HirFileId) -> FileId {
    file_id.original_file(database).file_id(database)
}

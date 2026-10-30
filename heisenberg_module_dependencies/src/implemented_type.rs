use ra_ap_hir::Crate;
use ra_ap_hir::HasSource;
use ra_ap_hir::Impl;
use ra_ap_ide_db::RootDatabase;
use ra_ap_vfs::FileId;

pub struct ImplementedType<'database> {
    pub database: &'database RootDatabase,
    pub implementation: Impl,
    pub krate: Crate,
}

impl ImplementedType<'_> {
    pub fn file(&self) -> Option<FileId> {
        let Self {
            database,
            implementation,
            krate,
        } = *self;

        implementation
            .self_ty(database)
            .strip_references()
            .as_adt()
            .filter(|adt| adt.module(database).krate(database) == krate)
            .and_then(|adt| adt.source(database))
            .map(|source| source.file_id.original_file(database).file_id(database))
    }
}

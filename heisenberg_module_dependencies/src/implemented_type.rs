use ra_ap_hir::Adt;
use ra_ap_hir::Crate;
use ra_ap_hir::HasSource;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Impl;
use ra_ap_hir::Trait;
use ra_ap_ide_db::RootDatabase;
use ra_ap_vfs::FileId;

use crate::impl_home::ImplHome;

fn written_file(database: &RootDatabase, file_id: HirFileId) -> FileId {
    file_id.original_file(database).file_id(database)
}

pub struct ImplementedType<'database> {
    pub database: &'database RootDatabase,
    pub implementation: Impl,
    pub krate: Crate,
}

impl ImplementedType<'_> {
    pub fn adt(&self) -> Option<Adt> {
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
    }

    pub fn home(&self) -> ImplHome {
        let database = self.database;

        self.adt()
            .and_then(|adt| adt.source(database))
            .map(|source| ImplHome::LocalType {
                file: written_file(database, source.file_id),
            })
            .or_else(|| {
                self.blanket_trait()
                    .and_then(|blanket_trait| blanket_trait.source(database))
                    .map(|source| ImplHome::LocalTraitBlanket {
                        file: written_file(database, source.file_id),
                    })
            })
            .unwrap_or(ImplHome::Foreign)
    }

    fn blanket_trait(&self) -> Option<Trait> {
        let Self {
            database,
            implementation,
            krate,
        } = *self;

        implementation
            .self_ty(database)
            .strip_references()
            .as_type_param(database)
            .and_then(|_| implementation.trait_(database))
            .filter(|blanket_trait| blanket_trait.module(database).krate(database) == krate)
    }
}

use ra_ap_hir::Adt;
use ra_ap_hir::Crate;
use ra_ap_hir::HasSource;
use ra_ap_hir::Impl;
use ra_ap_hir::Trait;
use ra_ap_hir::Type;
use ra_ap_ide_db::RootDatabase;

use crate::impl_home::ImplHome;
use crate::original_file_id::original_file_id;

pub struct ImplementedType<'database> {
    pub database: &'database RootDatabase,
    pub implementation: Impl,
    pub krate: Crate,
}

impl ImplementedType<'_> {
    pub fn adt(&self) -> Option<Adt> {
        self.implementation
            .self_ty(self.database)
            .strip_references()
            .as_adt()
            .filter(|adt| self.is_local_adt(*adt))
    }

    pub fn home(&self, trait_arguments: &[Type<'_>]) -> ImplHome {
        let database = self.database;

        self.adt()
            .and_then(|adt| adt.source(database))
            .map(|source| ImplHome::LocalType {
                file: original_file_id(database, source.file_id),
            })
            .or_else(|| {
                self.blanket_trait()
                    .and_then(|blanket_trait| blanket_trait.source(database))
                    .map(|source| ImplHome::LocalTraitBlanket {
                        file: original_file_id(database, source.file_id),
                    })
            })
            .or_else(|| {
                self.trait_object()
                    .and_then(|object_trait| object_trait.source(database))
                    .map(|source| ImplHome::LocalTraitObject {
                        file: original_file_id(database, source.file_id),
                    })
            })
            .or_else(|| {
                self.local_trait().and_then(|local_trait| {
                    local_trait
                        .source(database)
                        .map(|source| ImplHome::LocalTraitForeignType {
                            file: original_file_id(database, source.file_id),
                            trait_name: local_trait
                                .name(database)
                                .display(database, self.krate.edition(database))
                                .to_string(),
                        })
                })
            })
            .or_else(|| {
                self.local_type_argument(trait_arguments)
                    .and_then(|adt| adt.source(database))
                    .map(|source| ImplHome::LocalTypeArgument {
                        file: original_file_id(database, source.file_id),
                    })
            })
            .unwrap_or(ImplHome::Foreign)
    }

    fn blanket_trait(&self) -> Option<Trait> {
        let Self {
            database,
            implementation,
            ..
        } = *self;

        implementation
            .self_ty(database)
            .strip_references()
            .as_type_param(database)
            .and_then(|_| implementation.trait_(database))
            .filter(|blanket_trait| self.is_local_trait(*blanket_trait))
    }

    fn is_local_adt(&self, adt: Adt) -> bool {
        adt.module(self.database).krate(self.database) == self.krate
    }

    fn is_local_trait(&self, candidate: Trait) -> bool {
        candidate.module(self.database).krate(self.database) == self.krate
    }

    fn local_trait(&self) -> Option<Trait> {
        self.implementation
            .trait_(self.database)
            .filter(|implemented_trait| self.is_local_trait(*implemented_trait))
    }

    fn local_type_argument(&self, trait_arguments: &[Type<'_>]) -> Option<Adt> {
        let local: Vec<Adt> = trait_arguments
            .iter()
            .filter_map(|argument| argument.strip_references().as_adt())
            .filter(|adt| self.is_local_adt(*adt))
            .collect();

        match local.as_slice() {
            [adt] => Some(*adt),
            _ => None,
        }
    }

    fn trait_object(&self) -> Option<Trait> {
        self.implementation
            .self_ty(self.database)
            .strip_references()
            .as_dyn_trait()
            .filter(|object_trait| self.is_local_trait(*object_trait))
    }
}

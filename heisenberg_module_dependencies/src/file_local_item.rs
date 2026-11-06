use ra_ap_hir::ModuleDef;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_vfs::FileId;

use crate::outside_visibility::OutsideVisibility;

pub struct FileLocalItem<'item, 'database> {
    pub definition: ModuleDef,
    pub file_id: FileId,
    pub semantics: &'item Semantics<'database, RootDatabase>,
}

impl FileLocalItem<'_, '_> {
    pub fn is_file_local(&self) -> bool {
        self.is_private() && self.is_used_only_in_its_file()
    }

    pub fn is_private(&self) -> bool {
        self.definition.module(self.semantics.db).is_some()
            && !OutsideVisibility {
                database: self.semantics.db,
            }
            .definition_reaches_outside(self.definition)
    }

    fn is_used_only_in_its_file(&self) -> bool {
        let database = self.semantics.db;
        let references = Definition::from(self.definition)
            .usages(self.semantics)
            .all()
            .references;

        !references.is_empty()
            && references
                .keys()
                .all(|file| file.file_id(database) == self.file_id)
    }
}

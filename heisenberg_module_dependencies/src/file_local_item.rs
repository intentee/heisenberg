use ra_ap_vfs::FileId;

use crate::definition_usages::DefinitionUsages;
use crate::outside_visibility::OutsideVisibility;

pub struct FileLocalItem<'item, 'usages, 'database> {
    pub file_id: FileId,
    pub usages: &'item DefinitionUsages<'usages, 'database>,
}

impl FileLocalItem<'_, '_, '_> {
    pub fn is_file_local(&self) -> bool {
        self.is_private() && self.is_used_only_in_its_file()
    }

    pub fn is_private(&self) -> bool {
        let DefinitionUsages {
            definition,
            semantics,
            ..
        } = self.usages;

        definition.module(semantics.db).is_some()
            && !OutsideVisibility {
                database: semantics.db,
            }
            .definition_reaches_outside(*definition)
    }

    fn is_used_only_in_its_file(&self) -> bool {
        let database = self.usages.semantics.db;
        let references = &self.usages.all().references;

        !references.is_empty()
            && references
                .keys()
                .all(|file| file.file_id(database) == self.file_id)
    }
}

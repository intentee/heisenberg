use ra_ap_hir::Adt;
use ra_ap_hir::HasSource;
use ra_ap_hir::InFile;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::Trait;
use ra_ap_ide_db::RootDatabase;
use ra_ap_vfs::FileId;

use crate::original_file_id::original_file_id;

fn source_file<TOwner: HasSource>(owner: TOwner, database: &RootDatabase) -> Option<FileId> {
    owner
        .source(database)
        .map(|InFile { file_id, .. }| original_file_id(database, file_id))
}

pub enum LocalImplHome {
    TraitBlanket { owner: Trait },
    TraitForeignType { owner: Trait },
    TraitObject { owner: Trait },
    Type { owner: Adt },
    TypeArgument { owner: Adt },
}

impl LocalImplHome {
    pub fn file(&self, database: &RootDatabase) -> Option<FileId> {
        match self {
            Self::TraitBlanket { owner }
            | Self::TraitForeignType { owner }
            | Self::TraitObject { owner } => source_file(*owner, database),
            Self::Type { owner } | Self::TypeArgument { owner } => source_file(*owner, database),
        }
    }

    pub const fn owner(&self) -> ModuleDef {
        match self {
            Self::TraitBlanket { owner }
            | Self::TraitForeignType { owner }
            | Self::TraitObject { owner } => ModuleDef::Trait(*owner),
            Self::Type { owner } | Self::TypeArgument { owner } => ModuleDef::Adt(*owner),
        }
    }
}

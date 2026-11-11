use ra_ap_hir::Crate;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::EditionedFileId;
use ra_ap_paths::AbsPath;
use ra_ap_syntax::AstNode;
use ra_ap_vfs::FileId;

use crate::parsed_file::ParsedFile;

pub struct MeasuredFile<'project> {
    pub file_id: FileId,
    pub krate: Crate,
    pub path: &'project AbsPath,
}

impl MeasuredFile<'_> {
    pub fn parsed(&self, semantics: &Semantics<'_, RootDatabase>) -> ParsedFile {
        let source_file =
            EditionedFileId::new(semantics.db, self.file_id, self.krate.edition(semantics.db));

        ParsedFile {
            file_id: HirFileId::from(source_file),
            root: semantics.parse(source_file).syntax().clone(),
        }
    }
}

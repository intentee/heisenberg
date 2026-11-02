use std::collections::BTreeSet;

use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::span_for_offset;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::TextRange;
use ra_ap_vfs::FileId;

pub struct TokenOrigin<'origin, 'database> {
    pub own_files: &'origin BTreeSet<FileId>,
    pub semantics: &'origin Semantics<'database, RootDatabase>,
    pub workspace_files: &'origin BTreeSet<FileId>,
}

impl TokenOrigin<'_, '_> {
    pub fn is_foreign_text(&self, node: &SyntaxNode) -> bool {
        !self.is_written_in(node, self.own_files)
    }

    pub fn is_own_file(&self, file_id: FileId) -> bool {
        self.own_files.contains(&file_id)
    }

    pub fn is_own_text(&self, file_id: HirFileId, range: TextRange) -> bool {
        self.is_own_file(self.written_file(file_id, range))
    }

    pub fn is_workspace_text(&self, node: &SyntaxNode) -> bool {
        self.is_written_in(node, self.workspace_files)
    }

    pub fn written_range(&self, file_id: HirFileId, range: TextRange) -> FileRangeWrapper<FileId> {
        let database = self.semantics.db;

        file_id.macro_file().map_or_else(
            || FileRangeWrapper {
                file_id: file_id.original_file(database).file_id(database),
                range,
            },
            |call| {
                let (written, _) =
                    span_for_offset(database, call.expansion_span_map(database), range.start());

                FileRangeWrapper {
                    file_id: written.file_id.file_id(database),
                    range: written.range,
                }
            },
        )
    }

    fn is_written_in(&self, node: &SyntaxNode, files: &BTreeSet<FileId>) -> bool {
        let file_id = self.semantics.hir_file_for(node);

        node.descendants_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .filter(|token| !token.kind().is_trivia())
            .any(|token| files.contains(&self.written_file(file_id, token.text_range())))
    }

    fn written_file(&self, file_id: HirFileId, range: TextRange) -> FileId {
        file_id
            .span_map(self.semantics.db)
            .span_for_range(range)
            .anchor
            .file_id
            .file_id()
    }
}

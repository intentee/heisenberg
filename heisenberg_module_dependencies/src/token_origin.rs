use std::collections::BTreeSet;

use ra_ap_hir::HirFileId;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::TextRange;
use ra_ap_vfs::FileId;

pub struct TokenOrigin<'origin, 'database> {
    pub own_files: &'origin BTreeSet<FileId>,
    pub semantics: &'origin Semantics<'database, RootDatabase>,
}

impl TokenOrigin<'_, '_> {
    pub fn is_foreign_text(&self, node: &SyntaxNode) -> bool {
        let file_id = self.semantics.hir_file_for(node);

        !node
            .descendants_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .filter(|token| !token.kind().is_trivia())
            .any(|token| self.is_own_text(file_id, token.text_range()))
    }

    pub fn is_own_file(&self, file_id: FileId) -> bool {
        self.own_files.contains(&file_id)
    }

    pub fn is_own_text(&self, file_id: HirFileId, range: TextRange) -> bool {
        self.is_own_file(
            file_id
                .span_map(self.semantics.db)
                .span_for_range(range)
                .anchor
                .file_id
                .file_id(),
        )
    }
}

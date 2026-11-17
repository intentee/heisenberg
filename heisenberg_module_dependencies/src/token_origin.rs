use std::collections::BTreeSet;

use ra_ap_hir::Crate;
use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::Lookup;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::TextRange;
use ra_ap_vfs::FileId;

use heisenberg_expanded_code::code_tokens::code_tokens;
use heisenberg_expanded_code::generated_tokens::GeneratedTokens;
use heisenberg_source_locator::written_range::written_range;

pub struct TokenOrigin<'origin, 'database> {
    pub build_script_output_files: &'origin BTreeSet<FileId>,
    pub own_files: &'origin BTreeSet<FileId>,
    pub semantics: &'origin Semantics<'database, RootDatabase>,
    pub workspace_files: &'origin BTreeSet<FileId>,
}

impl TokenOrigin<'_, '_> {
    pub fn is_entirely_own_text(&self, node: &SyntaxNode) -> bool {
        let file_id = self.semantics.hir_file_for(node);

        code_tokens(node.descendants_with_tokens())
            .all(|token| self.is_own_text(file_id, token.text_range()))
    }

    pub fn is_foreign_text(&self, node: &SyntaxNode) -> bool {
        !self.is_written_in(node, self.own_files)
    }

    pub fn is_generated_by_a_workspace_macro(&self, node: &SyntaxNode) -> bool {
        let file_id = self.semantics.hir_file_for(node);

        code_tokens(node.descendants_with_tokens())
            .any(|token| self.is_generated_by_a_workspace_macro_at(file_id, token.text_range()))
    }

    pub fn is_own_file(&self, file_id: FileId) -> bool {
        self.own_files.contains(&file_id)
    }

    pub fn is_own_text(&self, file_id: HirFileId, range: TextRange) -> bool {
        self.is_text_of(file_id, range, self.own_files)
    }

    pub fn is_own_written_text(
        &self,
        file_id: HirFileId,
        range: TextRange,
        written: &FileRangeWrapper<FileId>,
    ) -> bool {
        GeneratedTokens {
            semantics: self.semantics,
        }
        .is_written_in(file_id, range, written, self.own_files)
    }

    pub fn is_workspace_code(&self, node: &SyntaxNode) -> bool {
        let file_id = self.semantics.hir_file_for(node);

        code_tokens(node.descendants_with_tokens())
            .any(|token| self.is_workspace_code_at(file_id, token.text_range()))
    }

    pub fn is_workspace_code_at(&self, file_id: HirFileId, range: TextRange) -> bool {
        self.is_written_by_the_workspace_at(file_id, range)
            || self.is_text_of(file_id, range, self.build_script_output_files)
    }

    pub fn is_workspace_crate(&self, krate: Crate) -> bool {
        self.workspace_files
            .contains(&krate.root_file(self.semantics.db))
    }

    pub fn is_workspace_range(&self, file_id: HirFileId, range: TextRange) -> bool {
        self.is_text_of(file_id, range, self.workspace_files)
    }

    pub fn is_workspace_text(&self, node: &SyntaxNode) -> bool {
        self.is_written_in(node, self.workspace_files)
    }

    pub fn is_written_by_the_workspace(&self, node: &SyntaxNode) -> bool {
        self.is_workspace_text(node) || self.is_generated_by_a_workspace_macro(node)
    }

    pub fn is_written_by_the_workspace_at(&self, file_id: HirFileId, range: TextRange) -> bool {
        self.is_workspace_range(file_id, range)
            || self.is_generated_by_a_workspace_macro_at(file_id, range)
    }

    fn is_generated_by_a_workspace_macro_at(&self, file_id: HirFileId, range: TextRange) -> bool {
        let database = self.semantics.db;

        GeneratedTokens {
            semantics: self.semantics,
        }
        .generating_proc_macro(file_id, range)
        .is_some_and(|call| self.is_workspace_crate(Crate::from(call.lookup(database).def.krate)))
    }

    fn is_text_of(&self, file_id: HirFileId, range: TextRange, files: &BTreeSet<FileId>) -> bool {
        GeneratedTokens {
            semantics: self.semantics,
        }
        .is_written_in(
            file_id,
            range,
            &written_range(self.semantics.db, file_id, range),
            files,
        )
    }

    fn is_written_in(&self, node: &SyntaxNode, files: &BTreeSet<FileId>) -> bool {
        let file_id = self.semantics.hir_file_for(node);

        code_tokens(node.descendants_with_tokens())
            .any(|token| self.is_text_of(file_id, token.text_range(), files))
    }
}

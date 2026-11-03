use std::collections::BTreeSet;

use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::Lookup;
use ra_ap_hir_expand::MacroCallId;
use ra_ap_hir_expand::MacroDefKind;
use ra_ap_hir_expand::span_for_offset;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::SourceDatabase;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::TextRange;
use ra_ap_vfs::FileId;

use crate::code_tokens::code_tokens;
use crate::original_file_id::original_file_id;

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
        !self.is_generated(file_id, range)
            && self.is_own_file(self.written_range(file_id, range).file_id)
    }

    pub fn is_workspace_text(&self, node: &SyntaxNode) -> bool {
        self.is_written_in(node, self.workspace_files)
    }

    pub fn written_range(&self, file_id: HirFileId, range: TextRange) -> FileRangeWrapper<FileId> {
        let database = self.semantics.db;

        file_id.macro_file().map_or_else(
            || FileRangeWrapper {
                file_id: original_file_id(database, file_id),
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

    fn is_generated(&self, file_id: HirFileId, range: TextRange) -> bool {
        let database = self.semantics.db;

        file_id.macro_file().is_some_and(|call| {
            let (written, context) =
                span_for_offset(database, call.expansion_span_map(database), range.start());

            Self::is_proc_macro_call(
                database,
                context.outer_expn(database).map(MacroCallId::from),
            ) || (Self::is_proc_macro_call(database, Some(call))
                && self
                    .semantics
                    .parse_or_expand(file_id)
                    .token_at_offset(range.start())
                    .right_biased()
                    .is_some_and(|token| {
                        database
                            .file_text(written.file_id.file_id(database))
                            .text(database)
                            .get(
                                usize::from(written.range.start())
                                    ..usize::from(written.range.end()),
                            )
                            != Some(token.text())
                    }))
        })
    }

    fn is_proc_macro_call(database: &RootDatabase, call: Option<MacroCallId>) -> bool {
        call.is_some_and(|call| {
            matches!(call.lookup(database).def.kind, MacroDefKind::ProcMacro(..))
        })
    }

    fn is_written_in(&self, node: &SyntaxNode, files: &BTreeSet<FileId>) -> bool {
        let file_id = self.semantics.hir_file_for(node);

        code_tokens(node.descendants_with_tokens()).any(|token| {
            !self.is_generated(file_id, token.text_range())
                && files.contains(&self.written_range(file_id, token.text_range()).file_id)
        })
    }
}

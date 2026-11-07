use std::collections::BTreeSet;
use std::iter;

use ra_ap_hir::Crate;
use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::Lookup;
use ra_ap_hir_expand::MacroCallId;
use ra_ap_hir_expand::MacroDefKind;
use ra_ap_hir_expand::span_for_offset;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::SyntaxToken;
use ra_ap_syntax::TextRange;
use ra_ap_vfs::FileId;

use crate::code_tokens::code_tokens;
use crate::macro_inputs::MacroInputs;
use crate::original_file_id::original_file_id;

fn span_sharing_text(token: &SyntaxToken, shares: impl Fn(&SyntaxToken) -> bool) -> String {
    let before: Vec<SyntaxToken> = iter::successors(token.prev_token(), SyntaxToken::prev_token)
        .filter(|neighbour| !neighbour.kind().is_trivia())
        .take_while(&shares)
        .collect();

    before
        .into_iter()
        .rev()
        .chain(iter::once(token.clone()))
        .chain(
            iter::successors(token.next_token(), SyntaxToken::next_token)
                .filter(|neighbour| !neighbour.kind().is_trivia())
                .take_while(&shares),
        )
        .map(|neighbour| neighbour.text().to_owned())
        .collect()
}

pub struct TokenOrigin<'origin, 'database> {
    pub macro_inputs: &'origin MacroInputs<'database>,
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
        let database = self.semantics.db;
        let file_id = self.semantics.hir_file_for(node);

        code_tokens(node.descendants_with_tokens()).any(|token| {
            self.generating_proc_macro(file_id, token.text_range())
                .is_some_and(|call| {
                    self.is_workspace_crate(Crate::from(call.lookup(database).def.krate))
                })
        })
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
        FileRangeWrapper {
            file_id: written_file,
            ..
        }: &FileRangeWrapper<FileId>,
    ) -> bool {
        !self.is_generated(file_id, range) && self.own_files.contains(written_file)
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

    fn generating_proc_macro(&self, file_id: HirFileId, range: TextRange) -> Option<MacroCallId> {
        let database = self.semantics.db;

        file_id.macro_file().and_then(|call| {
            call.expansion_span_map(database)
                .span_at(range.start())
                .ctx
                .outer_expn(database)
                .map(MacroCallId::from)
                .filter(|outer| Self::is_proc_macro_call(database, *outer))
                .or_else(|| {
                    (Self::is_proc_macro_call(database, call)
                        && self
                            .semantics
                            .parse_or_expand(file_id)
                            .token_at_offset(range.start())
                            .right_biased()
                            .is_some_and(|token| {
                                let spans = call.expansion_span_map(database);
                                let span = spans.span_at(token.text_range().start());
                                let input = self.macro_inputs.of(call);

                                !(input.holds(span, token.text())
                                    || input.holds(
                                        span,
                                        &span_sharing_text(&token, |neighbour| {
                                            spans.span_at(neighbour.text_range().start()) == span
                                        }),
                                    ))
                            }))
                    .then_some(call)
                })
        })
    }

    fn is_generated(&self, file_id: HirFileId, range: TextRange) -> bool {
        self.generating_proc_macro(file_id, range).is_some()
    }

    fn is_proc_macro_call(database: &RootDatabase, call: MacroCallId) -> bool {
        matches!(call.lookup(database).def.kind, MacroDefKind::ProcMacro(..))
    }

    fn is_text_of(&self, file_id: HirFileId, range: TextRange, files: &BTreeSet<FileId>) -> bool {
        !self.is_generated(file_id, range)
            && files.contains(&self.written_range(file_id, range).file_id)
    }

    fn is_written_in(&self, node: &SyntaxNode, files: &BTreeSet<FileId>) -> bool {
        let file_id = self.semantics.hir_file_for(node);

        code_tokens(node.descendants_with_tokens())
            .any(|token| self.is_text_of(file_id, token.text_range(), files))
    }
}

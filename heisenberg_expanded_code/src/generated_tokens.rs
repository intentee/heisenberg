use std::collections::BTreeSet;
use std::iter;
use std::ops::Range;

use ra_ap_hir::FileRange;
use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::Lookup;
use ra_ap_hir_expand::MacroCallId;
use ra_ap_hir_expand::MacroDefKind;
use ra_ap_hir_expand::name::AsName;
use ra_ap_hir_expand::span_for_offset;
use ra_ap_hir_expand::tt::LitKind;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::SourceDatabase;
use ra_ap_intern::sym;
use ra_ap_parser::LexedStr;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::AstToken;
use ra_ap_syntax::SyntaxKind;
use ra_ap_syntax::SyntaxToken;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::IsString;
use ra_ap_syntax_bridge::DocCommentDesugarMode;
use ra_ap_syntax_bridge::desugar_doc_comment_text;
use ra_ap_vfs::FileId;

use crate::code_tokens::code_tokens;

pub struct GeneratedTokens<'tokens, 'database> {
    pub semantics: &'tokens Semantics<'database, RootDatabase>,
}

impl GeneratedTokens<'_, '_> {
    #[must_use]
    pub fn generating_proc_macro(
        &self,
        file_id: HirFileId,
        range: TextRange,
    ) -> Option<MacroCallId> {
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
                            .is_some_and(|token| !self.is_written(call, &token)))
                    .then_some(call)
                })
        })
    }

    #[must_use]
    pub fn is_written_in(
        &self,
        file_id: HirFileId,
        range: TextRange,
        FileRangeWrapper {
            file_id: written_file,
            ..
        }: &FileRangeWrapper<FileId>,
        files: &BTreeSet<FileId>,
    ) -> bool {
        files.contains(written_file) && self.generating_proc_macro(file_id, range).is_none()
    }

    fn is_desugared_doc_comment(
        &self,
        call: MacroCallId,
        token: &SyntaxToken,
        FileRange { file_id, range }: FileRange,
    ) -> bool {
        let database = self.semantics.db;
        let spans = call.expansion_span_map(database);
        let span = spans.span_at(token.text_range().start());

        self.semantics
            .parse(file_id)
            .syntax()
            .covering_element(range)
            .into_token()
            .filter(|written| written.text_range() == range)
            .and_then(ast::AnyComment::cast)
            .is_some_and(|comment| {
                let (value, literal_kind) =
                    desugar_doc_comment_text(comment.text(), DocCommentDesugarMode::ProcMacro);

                token
                    .parent_ancestors()
                    .find_map(ast::Attr::cast)
                    .is_some_and(|attribute| {
                        comment.doc_kind() == Some(attribute.kind())
                            && code_tokens(attribute.syntax().descendants_with_tokens())
                                .all(|part| spans.span_at(part.text_range().start()) == span)
                            && attribute
                                .path()
                                .and_then(|path| path.as_single_name_ref())
                                .is_some_and(|name| *name.as_name().symbol() == sym::doc)
                            && attribute
                                .syntax()
                                .children()
                                .find_map(ast::KeyValueMeta::cast)
                                .and_then(|meta| meta.expr())
                                .and_then(|expression| {
                                    ast::Literal::cast(expression.syntax().clone())
                                })
                                .and_then(|literal| ast::String::cast(literal.token()))
                                .is_some_and(|string| {
                                    literal_kind == LitKind::Str
                                        && !string.is_raw()
                                        && string.text_without_quotes() == value.as_str()
                                })
                    })
            })
    }

    fn is_proc_macro_call(database: &RootDatabase, call: MacroCallId) -> bool {
        matches!(call.lookup(database).def.kind, MacroDefKind::ProcMacro(..))
    }

    fn is_split_float(
        &self,
        call: MacroCallId,
        token: &SyntaxToken,
        FileRange { file_id, .. }: FileRange,
        written: &str,
    ) -> bool {
        let database = self.semantics.db;
        let spans = call.expansion_span_map(database);
        let span = spans.span_at(token.text_range().start());
        let shares_the_span =
            |neighbour: &SyntaxToken| spans.span_at(neighbour.text_range().start()) == span;
        let lexed = LexedStr::new(file_id.edition(database), written);
        let first = iter::successors(token.prev_token(), SyntaxToken::prev_token)
            .take_while(&shares_the_span)
            .last()
            .unwrap_or_else(|| token.clone());
        let last = iter::successors(token.next_token(), SyntaxToken::next_token)
            .take_while(&shares_the_span)
            .last()
            .unwrap_or_else(|| token.clone());

        lexed.len() == 1
            && lexed.kind(0) == SyntaxKind::FLOAT_NUMBER
            && token.parent_ancestors().last().is_some_and(|root| {
                root.text().slice(TextRange::new(
                    first.text_range().start(),
                    last.text_range().end(),
                )) == written
            })
    }

    fn is_written(&self, call: MacroCallId, token: &SyntaxToken) -> bool {
        let database = self.semantics.db;
        let (written, _) = span_for_offset(
            database,
            call.expansion_span_map(database),
            token.text_range().start(),
        );

        database
            .file_text(written.file_id.file_id(database))
            .text(database)
            .get(Range::<usize>::from(written.range))
            .is_some_and(|text| {
                token.text() == text
                    || self.is_desugared_doc_comment(call, token, written)
                    || self.is_split_float(call, token, written, text)
            })
    }
}

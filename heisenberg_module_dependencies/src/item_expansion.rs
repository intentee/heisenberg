use ra_ap_hir::ExpandResult;
use ra_ap_hir::InFile;
use ra_ap_hir::MacroCallId;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use crate::macro_expansion::MacroExpansion;

pub enum ItemExpansion {
    Parsable {
        generated: Vec<ast::Item>,
        written: Vec<ast::Item>,
    },
    Unexpanded,
    Unparsable {
        call: MacroCallId,
    },
}

impl ItemExpansion {
    fn expanded(
        call: MacroCallId,
        root: &SyntaxNode,
        semantics: &Semantics<'_, RootDatabase>,
    ) -> Self {
        if (MacroExpansion {
            call,
            database: semantics.db,
        })
        .syntax_errors()
        .is_empty()
        {
            let (written, generated) = root
                .children()
                .filter_map(ast::Item::cast)
                .partition(|expanded| semantics.original_ast_node(expanded.clone()).is_some());

            Self::Parsable { generated, written }
        } else {
            Self::Unparsable { call }
        }
    }

    pub fn new(item: &ast::Item, semantics: &Semantics<'_, RootDatabase>) -> Self {
        semantics
            .expand_attr_macro(item)
            .and_then(
                |ExpandResult {
                     value: InFile { file_id, value },
                     ..
                 }| {
                    file_id
                        .macro_file()
                        .map(|call| Self::expanded(call, &value, semantics))
                },
            )
            .unwrap_or(Self::Unexpanded)
    }
}

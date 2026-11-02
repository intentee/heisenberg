use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;

use crate::macro_expansion::MacroExpansion;

pub enum ItemExpansion {
    Parsable {
        generated: Vec<ast::Item>,
        written: Vec<ast::Item>,
    },
    Unexpanded,
    Unparsable,
}

impl ItemExpansion {
    pub fn new(item: &ast::Item, semantics: &Semantics<'_, RootDatabase>) -> Self {
        match semantics.expand_attr_macro(item) {
            None => Self::Unexpanded,
            Some(expansion) => {
                if (MacroExpansion {
                    database: semantics.db,
                    file: expansion.value.file_id,
                })
                .is_parsable()
                {
                    let (written, generated) = expansion
                        .value
                        .value
                        .children()
                        .filter_map(ast::Item::cast)
                        .partition(|expanded| {
                            semantics.original_ast_node(expanded.clone()).is_some()
                        });

                    Self::Parsable { generated, written }
                } else {
                    Self::Unparsable
                }
            }
        }
    }
}

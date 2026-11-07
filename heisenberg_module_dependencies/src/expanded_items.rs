use ra_ap_cfg::CfgOptions;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use crate::defined_items::DefinedItems;
use crate::macro_expansion::MacroExpansion;
use crate::token_origin::TokenOrigin;

pub struct ExpandedItems<'items, 'database> {
    pub container: &'items SyntaxNode,
    pub options: &'items CfgOptions,
    pub semantics: &'items Semantics<'database, RootDatabase>,
    pub token_origin: &'items TokenOrigin<'items, 'database>,
}

impl ExpandedItems<'_, '_> {
    pub fn items(&self) -> Vec<ast::Item> {
        DefinedItems {
            container: self.container,
            options: self.options,
            semantics: self.semantics,
        }
        .items()
        .into_iter()
        .flat_map(|item| match item {
            ast::Item::MacroCall(call) => self.expansion(&call),
            defined => vec![defined],
        })
        .collect()
    }

    fn expansion(&self, call: &ast::MacroCall) -> Vec<ast::Item> {
        self.semantics
            .to_def(call)
            .filter(|expansion| {
                MacroExpansion {
                    call: *expansion,
                    database: self.semantics.db,
                }
                .syntax_errors()
                .is_empty()
            })
            .map_or_else(Vec::new, |expansion| {
                let root = self.semantics.parse_or_expand(HirFileId::from(expansion));

                ExpandedItems {
                    container: &root,
                    options: self.options,
                    semantics: self.semantics,
                    token_origin: self.token_origin,
                }
                .items()
                .into_iter()
                .filter(|item| self.token_origin.is_entirely_own_text(item.syntax()))
                .collect()
            })
    }
}

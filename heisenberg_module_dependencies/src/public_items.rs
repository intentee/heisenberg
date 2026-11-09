use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use crate::expanded_items::ExpandedItems;
use crate::item_visibility::ItemVisibility;
use crate::token_origin::TokenOrigin;

pub struct PublicItems<'items, 'database> {
    pub items: &'items [ast::Item],
    pub options: &'items CfgOptions,
    pub semantics: &'items Semantics<'database, RootDatabase>,
    pub token_origin: &'items TokenOrigin<'items, 'database>,
}

impl PublicItems<'_, '_> {
    pub fn non_constant(&self) -> Vec<ast::Item> {
        self.items
            .iter()
            .cloned()
            .flat_map(|item| match &item {
                ast::Item::ExternBlock(block) => block
                    .extern_item_list()
                    .map_or_else(Vec::new, |list| self.defined_in(list.syntax())),
                _ => vec![item],
            })
            .filter(|item| self.is_public_non_constant(item))
            .collect()
    }

    fn defined_in(&self, container: &SyntaxNode) -> Vec<ast::Item> {
        ExpandedItems {
            options: self.options,
            semantics: self.semantics,
            token_origin: self.token_origin,
        }
        .defined_in(container)
    }

    fn is_proc_macro(&self, function: &ast::Fn) -> bool {
        self.semantics
            .to_def(function)
            .is_some_and(|definition| definition.as_proc_macro(self.semantics.db).is_some())
    }

    fn is_public_non_constant(&self, item: &ast::Item) -> bool {
        match item {
            ast::Item::Const(_) | ast::Item::Module(_) => false,
            ast::Item::Fn(function) if self.is_proc_macro(function) => false,
            _ => ItemVisibility {
                item,
                semantics: self.semantics,
            }
            .is_public(),
        }
    }
}

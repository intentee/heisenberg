use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use crate::defined_items::DefinedItems;
use crate::item_visibility::ItemVisibility;

pub struct PublicItems<'items, 'database> {
    pub container: &'items SyntaxNode,
    pub options: &'items CfgOptions,
    pub semantics: &'items Semantics<'database, RootDatabase>,
}

impl PublicItems<'_, '_> {
    pub fn non_constant(&self) -> Vec<ast::Item> {
        self.defined_in(self.container)
            .into_iter()
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
        DefinedItems {
            container,
            options: self.options,
            semantics: self.semantics,
        }
        .items()
    }

    fn is_public_non_constant(&self, item: &ast::Item) -> bool {
        match item {
            ast::Item::Const(_) | ast::Item::Module(_) => false,
            _ => ItemVisibility {
                item,
                semantics: self.semantics,
            }
            .is_public(),
        }
    }
}

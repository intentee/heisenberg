use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use heisenberg_configured_code::configured_node::ConfiguredNode;

use crate::item_expansion::ItemExpansion;

pub struct DefinedItems<'items, 'database> {
    pub container: &'items SyntaxNode,
    pub options: &'items CfgOptions,
    pub semantics: &'items Semantics<'database, RootDatabase>,
}

impl DefinedItems<'_, '_> {
    pub fn items(&self) -> Vec<ast::Item> {
        self.container
            .children()
            .filter_map(ast::Item::cast)
            .flat_map(|item| self.defined(item))
            .collect()
    }

    fn defined(&self, item: ast::Item) -> Vec<ast::Item> {
        if (ConfiguredNode {
            node: item.syntax(),
            options: self.options,
        })
        .is_disabled()
        {
            Vec::new()
        } else {
            match ItemExpansion::new(&item, self.semantics) {
                ItemExpansion::Parsable { written, .. } => written
                    .into_iter()
                    .flat_map(|written| self.defined(written))
                    .collect(),
                ItemExpansion::Unexpanded => vec![item],
                ItemExpansion::Unparsable { .. } => Vec::new(),
            }
        }
    }
}

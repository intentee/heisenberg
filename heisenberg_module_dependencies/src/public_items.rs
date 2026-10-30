use ra_ap_cfg::CfgOptions;
use ra_ap_hir::HasVisibility;
use ra_ap_hir::Semantics;
use ra_ap_hir::Visibility;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasVisibility as _;

use crate::configured_node::ConfiguredNode;

pub struct PublicItems<'items, 'database> {
    pub container: &'items SyntaxNode,
    pub options: &'items CfgOptions,
    pub semantics: &'items Semantics<'database, RootDatabase>,
}

impl PublicItems<'_, '_> {
    pub fn non_constant(&self) -> Vec<ast::Item> {
        ConfiguredNode {
            node: self.container,
            options: self.options,
        }
        .enabled_items()
        .into_iter()
        .filter(|item| self.is_public_non_constant(item))
        .collect()
    }

    fn is_public_non_constant(&self, item: &ast::Item) -> bool {
        match item {
            ast::Item::AsmExpr(_)
            | ast::Item::Const(_)
            | ast::Item::ExternBlock(_)
            | ast::Item::ExternCrate(_)
            | ast::Item::Impl(_)
            | ast::Item::MacroCall(_)
            | ast::Item::Module(_)
            | ast::Item::Use(_) => false,
            ast::Item::MacroRules(definition) => self
                .semantics
                .to_macro_def(&ast::Macro::MacroRules(definition.clone()))
                .is_some_and(|definition| {
                    definition.visibility(self.semantics.db) == Visibility::Public
                }),
            ast::Item::Enum(_)
            | ast::Item::Fn(_)
            | ast::Item::MacroDef(_)
            | ast::Item::Static(_)
            | ast::Item::Struct(_)
            | ast::Item::Trait(_)
            | ast::Item::TypeAlias(_)
            | ast::Item::Union(_) => ast::AnyHasVisibility::cast(item.syntax().clone())
                .is_some_and(|definition| definition.visibility().is_some()),
        }
    }
}

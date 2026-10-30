use ra_ap_cfg::CfgOptions;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_intern::sym;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasAttrs;

use crate::configured_node::ConfiguredNode;

pub struct TestTargetItems<'items, 'database> {
    pub container: &'items SyntaxNode,
    pub options: &'items CfgOptions,
    pub semantics: &'items Semantics<'database, RootDatabase>,
}

impl TestTargetItems<'_, '_> {
    pub fn neither_tests_nor_imports(&self) -> Vec<ast::Item> {
        self.container
            .children()
            .filter_map(ast::Item::cast)
            .filter(|item| {
                !ConfiguredNode {
                    node: item.syntax(),
                    options: self.options,
                }
                .is_disabled()
            })
            .filter(|item| match item {
                ast::Item::Module(_) | ast::Item::Use(_) => false,
                ast::Item::Fn(function) => !self.is_test(function),
                ast::Item::AsmExpr(_)
                | ast::Item::Const(_)
                | ast::Item::Enum(_)
                | ast::Item::ExternBlock(_)
                | ast::Item::ExternCrate(_)
                | ast::Item::Impl(_)
                | ast::Item::MacroCall(_)
                | ast::Item::MacroDef(_)
                | ast::Item::MacroRules(_)
                | ast::Item::Static(_)
                | ast::Item::Struct(_)
                | ast::Item::Trait(_)
                | ast::Item::TypeAlias(_)
                | ast::Item::Union(_) => true,
            })
            .collect()
    }

    fn is_test(&self, function: &ast::Fn) -> bool {
        function
            .attrs()
            .filter_map(|attribute| attribute.path())
            .any(|path| {
                matches!(
                    self.semantics.resolve_path(&path),
                    Some(PathResolution::Def(ModuleDef::Macro(definition)))
                        if definition.name(self.semantics.db).symbol() == &sym::test
                )
            })
    }
}

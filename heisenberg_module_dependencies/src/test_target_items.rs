use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use heisenberg_module_graph::crate_target::CrateTarget;

use crate::attributed_item::AttributedItem;
use crate::expanded_items::ExpandedItems;
use crate::main_function::MainFunction;
use crate::token_origin::TokenOrigin;

pub struct TestTargetItems<'items, 'database> {
    pub container: &'items SyntaxNode,
    pub crate_target: &'items CrateTarget,
    pub options: &'items CfgOptions,
    pub semantics: &'items Semantics<'database, RootDatabase>,
    pub token_origin: &'items TokenOrigin<'items, 'database>,
}

impl TestTargetItems<'_, '_> {
    pub fn neither_tests_constants_nor_imports(&self) -> Vec<ast::Item> {
        ExpandedItems {
            container: self.container,
            options: self.options,
            semantics: self.semantics,
            token_origin: self.token_origin,
        }
        .items()
        .into_iter()
        .filter(|item| {
            let attributed = AttributedItem {
                item,
                semantics: self.semantics,
            };

            match item {
                ast::Item::Const(_) | ast::Item::Module(_) | ast::Item::Use(_) => false,
                ast::Item::Fn(function) => {
                    !attributed.has_test_attribute() && !self.is_entry_point(function)
                }
                ast::Item::AsmExpr(_)
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
            }
        })
        .collect()
    }

    fn is_entry_point(&self, function: &ast::Fn) -> bool {
        self.semantics.to_def(function).is_some_and(|definition| {
            MainFunction {
                crate_target: self.crate_target,
                database: self.semantics.db,
                function: definition,
            }
            .enters_its_target()
        })
    }
}

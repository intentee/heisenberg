use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use heisenberg_module_graph::test_harness::TestHarness;

use crate::attributed_item::AttributedItem;
use crate::defined_items::DefinedItems;

pub struct TestTargetItems<'items, 'database> {
    pub container: &'items SyntaxNode,
    pub options: &'items CfgOptions,
    pub semantics: &'items Semantics<'database, RootDatabase>,
    pub test_harness: TestHarness,
}

impl TestTargetItems<'_, '_> {
    pub fn neither_tests_constants_nor_imports(&self) -> Vec<ast::Item> {
        DefinedItems {
            container: self.container,
            options: self.options,
            semantics: self.semantics,
        }
        .items()
        .into_iter()
        .filter(|item| {
            let attributed = AttributedItem {
                item,
                semantics: self.semantics,
            };

            !attributed.has_procedural_attribute()
                && match item {
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
        self.test_harness == TestHarness::Absent
            && self
                .semantics
                .to_def(function)
                .is_some_and(|definition| definition.is_main(self.semantics.db))
    }
}

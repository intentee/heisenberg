use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::ast;

use heisenberg_module_graph::crate_target::CrateTarget;

use crate::attributed_item::AttributedItem;
use crate::main_function::MainFunction;
use crate::path_resolutions::PathResolutions;

pub struct TestTargetItems<'items, 'database> {
    pub crate_target: &'items CrateTarget,
    pub items: &'items [ast::Item],
    pub paths: &'items PathResolutions<'items, 'database>,
    pub semantics: &'items Semantics<'database, RootDatabase>,
}

impl TestTargetItems<'_, '_> {
    pub fn neither_tests_constants_nor_imports(&self) -> Vec<ast::Item> {
        self.items
            .iter()
            .filter(|item| {
                let attributed = AttributedItem {
                    item,
                    paths: self.paths,
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
            .cloned()
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

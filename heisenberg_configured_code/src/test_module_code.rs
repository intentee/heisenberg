use ra_ap_hir::InFile;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::SyntaxNodePtr;
use ra_ap_syntax::ast;

use crate::ancestry::Ancestry;
use crate::configured_node::ConfiguredNode;
use crate::test_modules::TestModules;

pub struct TestModuleCode<'code, 'database> {
    pub modules: &'code TestModules,
    pub semantics: &'code Semantics<'database, RootDatabase>,
}

impl TestModuleCode<'_, '_> {
    #[must_use]
    pub fn contains(&self, node: &SyntaxNode) -> bool {
        self.modules.enclosing.get_or_init(
            InFile::new(self.semantics.hir_file_for(node), SyntaxNodePtr::new(node)),
            || {
                ast::Module::cast(node.clone()).is_some_and(|module| self.is_test_module(&module))
                    || Ancestry {
                        node,
                        semantics: self.semantics,
                    }
                    .enclosing()
                    .is_some_and(|enclosing| self.contains(&enclosing))
            },
        )
    }

    fn is_test_module(&self, module: &ast::Module) -> bool {
        let database = self.semantics.db;

        self.semantics.to_def(module).is_some_and(|definition| {
            let krate = definition.krate(database);

            ConfiguredNode {
                node: module.syntax(),
                options: &self.modules.production_options(database, krate),
            }
            .is_disabled()
        })
    }
}

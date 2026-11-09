use std::collections::BTreeSet;
use std::sync::Arc;

use ra_ap_hir::InFile;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::SyntaxNodePtr;
use ra_ap_syntax::ast;
use ra_ap_vfs::FileId;

use crate::ancestry::Ancestry;
use crate::configured_node::ConfiguredNode;
use crate::original_file_id::original_file_id;
use crate::production_options::production_options;
use crate::test_modules::TestModules;

pub struct TestCode<'code, 'database> {
    pub files: &'code BTreeSet<FileId>,
    pub modules: &'code TestModules,
    pub semantics: &'code Semantics<'database, RootDatabase>,
}

impl TestCode<'_, '_> {
    pub fn contains(&self, node: &SyntaxNode) -> bool {
        self.files.contains(&original_file_id(
            self.semantics.db,
            self.semantics.hir_file_for(node),
        )) || self.is_in_test_module(node)
    }

    fn is_in_test_module(&self, node: &SyntaxNode) -> bool {
        self.modules.enclosing.get_or_init(
            InFile::new(self.semantics.hir_file_for(node), SyntaxNodePtr::new(node)),
            || {
                ast::Module::cast(node.clone()).is_some_and(|module| self.is_test_module(&module))
                    || Ancestry {
                        node,
                        semantics: self.semantics,
                    }
                    .enclosing()
                    .is_some_and(|enclosing| self.is_in_test_module(&enclosing))
            },
        )
    }

    fn is_test_module(&self, module: &ast::Module) -> bool {
        let database = self.semantics.db;

        self.semantics.to_def(module).is_some_and(|definition| {
            let krate = definition.krate(database);

            ConfiguredNode {
                node: module.syntax(),
                options: &self
                    .modules
                    .production_options
                    .get_or_init(krate, || Arc::new(production_options(krate.cfg(database)))),
            }
            .is_disabled()
        })
    }
}

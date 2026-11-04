use std::collections::BTreeSet;

use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;
use ra_ap_vfs::FileId;

use crate::ancestry::Ancestry;
use crate::configured_node::ConfiguredNode;
use crate::original_file_id::original_file_id;
use crate::production_options::production_options;

pub struct TestCode<'code, 'database> {
    pub files: &'code BTreeSet<FileId>,
    pub semantics: &'code Semantics<'database, RootDatabase>,
}

impl TestCode<'_, '_> {
    pub fn contains(&self, node: &SyntaxNode) -> bool {
        self.files.contains(&original_file_id(
            self.semantics.db,
            self.semantics.hir_file_for(node),
        )) || Ancestry {
            node,
            semantics: self.semantics,
        }
        .nodes()
        .into_iter()
        .filter_map(ast::Module::cast)
        .any(|module| self.is_test_module(&module))
    }

    fn is_test_module(&self, module: &ast::Module) -> bool {
        let database = self.semantics.db;

        self.semantics.to_def(module).is_some_and(|definition| {
            ConfiguredNode {
                node: module.syntax(),
                options: &production_options(definition.krate(database).cfg(database)),
            }
            .is_disabled()
        })
    }
}

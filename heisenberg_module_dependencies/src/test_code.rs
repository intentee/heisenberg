use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::HashMap;

use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Crate;
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
    in_test_module: RefCell<HashMap<SyntaxNode, bool>>,
    production_options: RefCell<HashMap<Crate, CfgOptions>>,
}

impl<'code, 'database> TestCode<'code, 'database> {
    pub fn new(
        files: &'code BTreeSet<FileId>,
        semantics: &'code Semantics<'database, RootDatabase>,
    ) -> Self {
        Self {
            files,
            semantics,
            in_test_module: RefCell::default(),
            production_options: RefCell::default(),
        }
    }

    pub fn contains(&self, node: &SyntaxNode) -> bool {
        self.files.contains(&original_file_id(
            self.semantics.db,
            self.semantics.hir_file_for(node),
        )) || self.is_in_test_module(node)
    }

    fn is_in_test_module(&self, node: &SyntaxNode) -> bool {
        let known = self.in_test_module.borrow().get(node).copied();

        known.unwrap_or_else(|| {
            let inside = ast::Module::cast(node.clone())
                .is_some_and(|module| self.is_test_module(&module))
                || Ancestry {
                    node,
                    semantics: self.semantics,
                }
                .enclosing()
                .is_some_and(|enclosing| self.is_in_test_module(&enclosing));

            self.in_test_module
                .borrow_mut()
                .insert(node.clone(), inside);

            inside
        })
    }

    fn is_test_module(&self, module: &ast::Module) -> bool {
        let database = self.semantics.db;

        self.semantics.to_def(module).is_some_and(|definition| {
            let krate = definition.krate(database);
            let mut options = self.production_options.borrow_mut();

            ConfiguredNode {
                node: module.syntax(),
                options: options
                    .entry(krate)
                    .or_insert_with(|| production_options(krate.cfg(database))),
            }
            .is_disabled()
        })
    }
}

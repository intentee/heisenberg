use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::ast;

use heisenberg_module_graph::crate_target::CrateTarget;

use crate::main_function::MainFunction;

pub struct EntryPoint<'entry, 'database> {
    pub crate_target: &'entry CrateTarget,
    pub semantics: &'entry Semantics<'database, RootDatabase>,
}

impl EntryPoint<'_, '_> {
    pub fn is_written_as(&self, function: &ast::Fn) -> bool {
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

use ra_ap_hir::HasVisibility;
use ra_ap_hir::Macro;
use ra_ap_hir::MacroKind;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;

use crate::local_macro::LocalMacro;

pub struct LocalMacroImport<'import, 'database> {
    pub path: &'import ast::Path,
    pub semantics: &'import Semantics<'database, RootDatabase>,
}

impl LocalMacroImport<'_, '_> {
    pub fn imported_macro(&self) -> Option<LocalMacro> {
        self.macro_definition().map(|definition| {
            if definition.visibility(self.semantics.db).is_public() {
                LocalMacro::Exported
            } else {
                LocalMacro::CrateLocal
            }
        })
    }

    fn macro_definition(&self) -> Option<Macro> {
        let database = self.semantics.db;

        match self.semantics.resolve_path(self.path) {
            Some(PathResolution::Def(ModuleDef::Macro(definition)))
                if definition.kind(database) == MacroKind::Declarative
                    && self
                        .semantics
                        .scope(self.path.syntax())
                        .is_some_and(|scope| scope.module() == definition.module(database)) =>
            {
                Some(definition)
            }
            _ => None,
        }
    }
}

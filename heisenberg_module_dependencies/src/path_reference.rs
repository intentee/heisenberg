use ra_ap_hir::Crate;
use ra_ap_hir::Module;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::name::AsName;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use crate::module_paths::ModulePaths;
use crate::path_reference_outcome::PathReferenceOutcome;
use crate::referenced_modules::ReferencedModules;
use crate::token_origin::TokenOrigin;
use crate::unresolved_import_binding::UnresolvedImportBinding;

pub struct PathReference<'reference, 'database> {
    pub krate: Crate,
    pub module_paths: &'reference ModulePaths<'database>,
    pub path: &'reference ast::Path,
    pub scope: &'reference SyntaxNode,
    pub semantics: &'reference Semantics<'database, RootDatabase>,
    pub token_origin: &'reference TokenOrigin<'reference, 'database>,
}

impl PathReference<'_, '_> {
    pub fn outcome(&self) -> PathReferenceOutcome {
        match self.semantics.resolve_path(self.path) {
            Some(PathResolution::Def(definition)) => self
                .semantics
                .scope(self.path.syntax())
                .map_or(PathReferenceOutcome::Unresolved, |scope| {
                    self.dependency(scope.module(), definition)
                }),
            Some(_) => PathReferenceOutcome::Irrelevant,
            None if self.names_an_attribute_or_a_macro_call()
                || self.is_type_anchor()
                || self.token_origin.is_foreign_text(self.path.syntax()) =>
            {
                PathReferenceOutcome::Irrelevant
            }
            None => self.path.qualifier().map_or_else(
                || self.single_segment(),
                |qualifier| self.qualified(&qualifier),
            ),
        }
    }

    fn dependency(&self, from: Module, definition: ModuleDef) -> PathReferenceOutcome {
        let database = self.semantics.db;
        let target = match definition {
            ModuleDef::Module(module) if from.path_to_root(database).contains(&module) => None,
            ModuleDef::Module(module) => Some(module),
            other => other.module(database),
        };

        let from = self.module_paths.path(from);

        target
            .filter(|to| to.krate(database) == self.krate)
            .map(|to| self.module_paths.path(to))
            .filter(|to| *to != from)
            .map_or(PathReferenceOutcome::Irrelevant, |to| {
                PathReferenceOutcome::Dependency(ReferencedModules { from, to })
            })
    }

    fn is_type_anchor(&self) -> bool {
        self.path
            .segment()
            .is_some_and(|segment| segment.type_anchor().is_some())
    }

    fn names_an_attribute_or_a_macro_call(&self) -> bool {
        self.path
            .syntax()
            .ancestors()
            .find(|ancestor| !ast::Path::can_cast(ancestor.kind()))
            .is_some_and(|ancestor| {
                ast::Meta::can_cast(ancestor.kind()) || ast::MacroCall::can_cast(ancestor.kind())
            })
    }

    fn qualified(&self, qualifier: &ast::Path) -> PathReferenceOutcome {
        match self.semantics.resolve_path(qualifier) {
            None
            | Some(
                PathResolution::Def(
                    ModuleDef::Adt(_)
                    | ModuleDef::BuiltinType(_)
                    | ModuleDef::Trait(_)
                    | ModuleDef::TypeAlias(_),
                )
                | PathResolution::SelfType(_)
                | PathResolution::TypeParam(_),
            ) => PathReferenceOutcome::Irrelevant,
            Some(PathResolution::Def(ModuleDef::Module(module)))
                if module.krate(self.semantics.db) != self.krate =>
            {
                PathReferenceOutcome::Irrelevant
            }
            Some(_) => PathReferenceOutcome::Unresolved,
        }
    }

    fn single_segment(&self) -> PathReferenceOutcome {
        let bound_by_unresolved_import = self
            .path
            .segment()
            .and_then(|segment| segment.name_ref())
            .is_some_and(|name| {
                self.semantics.scope(self.scope).is_some_and(|scope| {
                    UnresolvedImportBinding {
                        name: &name.as_name(),
                        scope: &scope,
                    }
                    .binds()
                })
            });

        if bound_by_unresolved_import {
            PathReferenceOutcome::Irrelevant
        } else {
            PathReferenceOutcome::Unresolved
        }
    }
}

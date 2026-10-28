use ra_ap_hir::Crate;
use ra_ap_hir::Module;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxKind;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use crate::import_binding::ImportBinding;
use crate::path_reference_outcome::PathReferenceOutcome;
use crate::referenced_modules::ReferencedModules;
use crate::token_origin::TokenOrigin;

pub struct PathReference<'reference, 'database> {
    pub krate: Crate,
    pub path: &'reference ast::Path,
    pub scope: &'reference SyntaxNode,
    pub semantics: &'reference Semantics<'database, RootDatabase>,
    pub token_origin: &'reference TokenOrigin<'reference, 'database>,
}

impl PathReference<'_, '_> {
    #[expect(
        clippy::expect_used,
        reason = "rust-analyzer knows the scope of every node of a parsed source file or macro expansion"
    )]
    pub fn outcome(&self) -> PathReferenceOutcome {
        let database = self.semantics.db;
        let from = self
            .semantics
            .scope(self.path.syntax())
            .expect("the path belongs to a parsed source file or macro expansion")
            .module()
            .nearest_non_block_module(database);

        match self.semantics.resolve_path(self.path) {
            Some(PathResolution::Def(definition)) => self.dependency(from, definition),
            Some(_) => PathReferenceOutcome::Irrelevant,
            None if self.is_attribute_name()
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

        target
            .map(|module| module.nearest_non_block_module(database))
            .filter(|to| to.krate(database) == self.krate && *to != from)
            .map_or(PathReferenceOutcome::Irrelevant, |to| {
                PathReferenceOutcome::Dependency(ReferencedModules { from, to })
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
        if self
            .path
            .segment()
            .and_then(|segment| segment.name_ref())
            .is_some_and(|name| {
                ImportBinding {
                    krate: self.krate,
                    semantics: self.semantics,
                }
                .imports_from_another_crate(self.scope, &name)
            })
        {
            PathReferenceOutcome::Irrelevant
        } else {
            PathReferenceOutcome::Unresolved
        }
    }

    fn is_type_anchor(&self) -> bool {
        self.path
            .segment()
            .is_some_and(|segment| segment.type_anchor().is_some())
    }

    fn is_attribute_name(&self) -> bool {
        self.path
            .syntax()
            .ancestors()
            .find(|ancestor| ancestor.kind() != SyntaxKind::PATH)
            .is_some_and(|ancestor| ast::Meta::can_cast(ancestor.kind()))
    }
}

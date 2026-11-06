use ra_ap_hir::Adt;
use ra_ap_hir::Semantics;
use ra_ap_hir::Type;
use ra_ap_hir::Variant;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_ide_db::search::FileReference;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;

use crate::token_origin::TokenOrigin;

pub struct FieldOnlyParameter<'parameter, 'database> {
    pub binding: &'parameter ast::IdentPat,
    pub semantics: &'parameter Semantics<'database, RootDatabase>,
    pub token_origin: &'parameter TokenOrigin<'parameter, 'database>,
}

impl<'database> FieldOnlyParameter<'_, 'database> {
    pub fn destructurable_type(&self) -> Option<Type<'database>> {
        self.semantics.to_def(self.binding).and_then(|local| {
            let held = local.ty(self.semantics.db).strip_references();
            let references = Definition::Local(local)
                .usages(self.semantics)
                .all()
                .references;

            (!references.is_empty()
                && references
                    .values()
                    .flatten()
                    .all(|reference| self.reads_a_field_of(reference, &held)))
            .then_some(held)
        })
    }

    fn field_access(FileReference { name, .. }: &FileReference) -> Option<ast::FieldExpr> {
        let path_expression = name
            .as_name_ref()
            .and_then(|name_ref| name_ref.syntax().parent())
            .and_then(ast::PathSegment::cast)
            .and_then(|segment| segment.parent_path().syntax().parent())
            .and_then(ast::PathExpr::cast)?;

        path_expression
            .syntax()
            .parent()
            .and_then(ast::FieldExpr::cast)
            .filter(|access| {
                access
                    .expr()
                    .is_some_and(|receiver| receiver.syntax() == path_expression.syntax())
            })
    }

    fn reads_a_field_of(&self, reference: &FileReference, held: &Type<'_>) -> bool {
        Self::field_access(reference)
            .filter(|access| {
                access.name_ref().is_some_and(|field_name| {
                    self.token_origin.is_own_text(
                        self.semantics.hir_file_for(access.syntax()),
                        field_name.syntax().text_range(),
                    )
                })
            })
            .and_then(|access| self.semantics.resolve_field(&access))
            .is_some_and(|resolved| {
                resolved.left().map_or_else(
                    || held.is_tuple(),
                    |field| {
                        matches!(
                            field.parent_def(self.semantics.db),
                            Variant::Struct(owner) if held.as_adt() == Some(Adt::Struct(owner))
                        )
                    },
                )
            })
    }
}

use ra_ap_hir::Adt;
use ra_ap_hir::Crate;
use ra_ap_hir::FindPathConfig;
use ra_ap_hir::Module;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::Semantics;
use ra_ap_hir::Type;
use ra_ap_hir::Variant;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_ide_db::famous_defs::FamousDefs;
use ra_ap_ide_db::search::FileReference;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;

use crate::token_origin::TokenOrigin;

pub struct FieldOnlyParameter<'parameter, 'database> {
    pub binding: &'parameter ast::IdentPat,
    pub krate: Crate,
    pub semantics: &'parameter Semantics<'database, RootDatabase>,
    pub token_origin: &'parameter TokenOrigin<'parameter, 'database>,
}

impl<'database> FieldOnlyParameter<'_, 'database> {
    pub fn destructurable_type(&self) -> Option<Type<'database>> {
        self.semantics.to_def(self.binding).and_then(|local| {
            let database = self.semantics.db;
            let declared = local.ty(database);
            let held = declared.strip_references();

            Definition::Local(local)
                .usages(self.semantics)
                .all()
                .references
                .values()
                .flatten()
                .map(|reference| self.read_field_type(reference, &held))
                .collect::<Option<Vec<Type<'database>>>>()
                .filter(|read_field_types| {
                    !read_field_types.is_empty()
                        && (declared.is_reference()
                            || !self.implements_drop(&held)
                            || read_field_types
                                .iter()
                                .all(|read_field_type| read_field_type.is_copy(database)))
                })
                .filter(|_| self.is_nameable_in(local.module(database), &held))
                .map(|_| held)
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

    fn implements_drop(&self, held: &Type<'_>) -> bool {
        FamousDefs(self.semantics, self.krate)
            .core_ops_Drop()
            .is_some_and(|drop| held.impls_trait(self.semantics.db, drop, &[]))
    }

    fn is_nameable_in(&self, module: Module, held: &Type<'_>) -> bool {
        held.as_adt().is_none_or(|adt| {
            module
                .find_path(
                    self.semantics.db,
                    ModuleDef::Adt(adt),
                    FindPathConfig {
                        allow_unstable: false,
                        prefer_absolute: false,
                        prefer_no_std: false,
                        prefer_prelude: false,
                    },
                )
                .is_some()
        })
    }

    fn read_field_type(
        &self,
        reference: &FileReference,
        held: &Type<'_>,
    ) -> Option<Type<'database>> {
        let database = self.semantics.db;

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
            .and_then(|resolved| {
                resolved.either(
                    |field| {
                        matches!(
                            field.parent_def(database),
                            Variant::Struct(owner) if held.as_adt() == Some(Adt::Struct(owner))
                        )
                        .then(|| field.ty(database))
                    },
                    |tuple_field| held.is_tuple().then(|| tuple_field.ty(database)),
                )
            })
    }
}

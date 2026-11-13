use ra_ap_hir::Adt;
use ra_ap_hir::AsAssocItem;
use ra_ap_hir::AssocItemContainer;
use ra_ap_hir::Function;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_hir::Type;
use ra_ap_hir::TypeInfo;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_ide_db::search::FileReference;
use ra_ap_ide_db::search::ReferenceCategory;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasArgList;
use ra_ap_syntax::ast::HasName;

use crate::constants::BORROWING_RESULT_METHODS;
use crate::constants::ERROR_DROPPING_RESULT_METHODS;
use crate::constants::RESULT_ERROR_VARIANT;
use crate::constants::RESULT_VALUE_VARIANT;
use crate::error_types::ErrorTypes;
use crate::method_call_resolutions::MethodCallResolutions;
use crate::path_resolutions::PathResolutions;
use crate::standard_items::StandardItems;
use crate::substituted_resolution::SubstitutedResolution;

fn is_bodiless_parameter(node: &SyntaxNode) -> bool {
    node.ancestors()
        .find_map(ast::Param::cast)
        .and_then(|parameter| parameter.syntax().parent())
        .and_then(|list| list.parent())
        .is_some_and(|owner| {
            ast::FnPtrType::can_cast(owner.kind())
                || ast::Fn::cast(owner).is_some_and(|function| function.body().is_none())
        })
}

pub struct ErrorDiscards<'discards, 'database> {
    pub error_types: &'discards ErrorTypes<'discards, 'database>,
    pub method_calls: &'discards MethodCallResolutions<'discards, 'database>,
    pub paths: &'discards PathResolutions<'discards, 'database>,
    pub semantics: &'discards Semantics<'database, RootDatabase>,
    pub standard_items: &'discards StandardItems,
}

impl<'database> ErrorDiscards<'_, 'database> {
    pub fn discarded(&self, node: &SyntaxNode) -> Option<SyntaxNode> {
        ast::WildcardPat::cast(node.clone())
            .and_then(|wildcard| self.wildcard(&ast::Pat::WildcardPat(wildcard)))
            .or_else(|| ast::RestPat::cast(node.clone()).and_then(|rest| self.rest(&rest)))
            .or_else(|| {
                ast::UnderscoreExpr::cast(node.clone())
                    .and_then(|underscore| self.underscore(&underscore))
            })
            .or_else(|| {
                ast::TupleStructPat::cast(node.clone())
                    .and_then(|pattern| self.value_only(&pattern))
            })
            .or_else(|| {
                ast::MethodCallExpr::cast(node.clone()).and_then(|call| self.method_call(&call))
            })
            .or_else(|| {
                ast::PathExpr::cast(node.clone()).and_then(|path| self.function_value(&path))
            })
            .or_else(|| {
                ast::ForExpr::cast(node.clone()).and_then(|repetition| self.iterated(&repetition))
            })
            .or_else(|| ast::IdentPat::cast(node.clone()).and_then(|binding| self.unread(&binding)))
    }

    fn function_value(&self, path: &ast::PathExpr) -> Option<SyntaxNode> {
        let database = self.semantics.db;

        path.path()
            .and_then(|written| match self.paths.resolve(&written) {
                Some(PathResolution::Def(ModuleDef::Function(function))) => Some(function),
                _ => None,
            })
            .filter(|function| {
                *function == self.standard_items.drop
                    || self.is_result_method(*function, &ERROR_DROPPING_RESULT_METHODS)
                    || self.is_result_into_iter(*function)
            })
            .and_then(|_| {
                self.semantics
                    .type_of_expr(&ast::Expr::PathExpr(path.clone()))
            })
            .and_then(|TypeInfo { original, .. }| original.as_callable(database))
            .and_then(|callable| callable.params().into_iter().next())
            .filter(|parameter| self.error_types.is_discardable(parameter.ty()))
            .map(|_| path.syntax().clone())
    }

    fn is_behind_a_reference(&self, pattern: &ast::Pat) -> bool {
        pattern
            .syntax()
            .ancestors()
            .map_while(ast::Pat::cast)
            .any(|ancestor| {
                matches!(ancestor, ast::Pat::RefPat(_))
                    || !self.semantics.pattern_adjustments(&ancestor).is_empty()
            })
    }

    fn is_iterator_method(&self, function: Function, name: &str) -> bool {
        let database = self.semantics.db;

        function.name(database).as_str() == name
            && function
                .as_assoc_item(database)
                .and_then(|item| item.container_or_implemented_trait(database))
                == Some(self.standard_items.iterator)
    }

    fn is_place(&self, expression: &ast::Expr) -> bool {
        match expression {
            ast::Expr::ParenExpr(inner) => inner.expr().is_some_and(|inner| self.is_place(&inner)),
            ast::Expr::PathExpr(path) => path.path().is_some_and(|written| {
                matches!(
                    self.paths.resolve(&written),
                    Some(PathResolution::Local(_) | PathResolution::Def(ModuleDef::Static(_)))
                )
            }),
            ast::Expr::FieldExpr(field) => field.expr().is_some_and(|owner| self.is_place(&owner)),
            ast::Expr::IndexExpr(_) => true,
            ast::Expr::PrefixExpr(prefix) => prefix.op_kind() == Some(ast::UnaryOp::Deref),
            _ => false,
        }
    }

    fn is_result_into_iter(&self, function: Function) -> bool {
        let database = self.semantics.db;

        matches!(
            function.as_assoc_item(database).map(|item| item.container(database)),
            Some(AssocItemContainer::Impl(implementation))
                if implementation.trait_(database) == Some(self.standard_items.into_iterator)
                    && implementation.self_ty(database).as_adt()
                        == Some(Adt::Enum(self.standard_items.result))
        )
    }

    fn is_result_method(&self, function: Function, names: &[&str]) -> bool {
        let database = self.semantics.db;

        names.contains(&function.name(database).as_str())
            && matches!(
                function.as_assoc_item(database).map(|item| item.container(database)),
                Some(AssocItemContainer::Impl(implementation))
                    if implementation.trait_(database).is_none()
                        && implementation.self_ty(database).as_adt()
                            == Some(Adt::Enum(self.standard_items.result))
            )
    }

    fn is_result_variant(&self, pattern: &ast::TupleStructPat, name: &str) -> bool {
        let database = self.semantics.db;

        pattern.path().is_some_and(|written| {
            matches!(
                self.paths.resolve(&written),
                Some(PathResolution::Def(ModuleDef::EnumVariant(variant)))
                    if variant.parent_enum(database) == self.standard_items.result
                        && variant.name(database).as_str() == name
            )
        })
    }

    fn iterated(&self, repetition: &ast::ForExpr) -> Option<SyntaxNode> {
        repetition
            .iterable()
            .filter(|iterable| self.owns_a_result_error(iterable))
            .map(|iterable| iterable.syntax().clone())
    }

    fn method_call(&self, call: &ast::MethodCallExpr) -> Option<SyntaxNode> {
        let database = self.semantics.db;

        self.method_calls.resolve(call).and_then(
            |SubstitutedResolution {
                 resolution: function,
                 ..
             }| {
                call.receiver()
                    .filter(|receiver| {
                        ((self.is_result_method(function, &ERROR_DROPPING_RESULT_METHODS)
                            || self.is_result_into_iter(function))
                            && self.owns_a_result_error(receiver))
                            || (self.is_result_method(function, &BORROWING_RESULT_METHODS)
                                && !self.is_place(receiver)
                                && self.owns_a_result_error(receiver))
                            || (self.is_iterator_method(function, "flatten")
                                && self
                                    .semantics
                                    .type_of_expr(receiver)
                                    .and_then(|TypeInfo { original, .. }| {
                                        original.iterator_item(database)
                                    })
                                    .is_some_and(|item| {
                                        self.error_types.owned_result_error(&item).is_some()
                                    }))
                            || (self.is_iterator_method(function, "flat_map")
                                && call
                                    .arg_list()
                                    .and_then(|arguments| arguments.args().next())
                                    .and_then(|mapping| self.semantics.type_of_expr(&mapping))
                                    .and_then(|TypeInfo { original, .. }| {
                                        original.as_callable(database)
                                    })
                                    .is_some_and(|mapping| {
                                        self.error_types
                                            .owned_result_error(&mapping.return_type())
                                            .is_some()
                                    }))
                    })
                    .and_then(|_| call.name_ref())
                    .map(|name| name.syntax().clone())
            },
        )
    }

    fn omitted_by(&self, parent: &SyntaxNode) -> Vec<Type<'database>> {
        let database = self.semantics.db;

        ast::TupleStructPat::cast(parent.clone())
            .map(|pattern| {
                (self.is_result_variant(&pattern, RESULT_ERROR_VARIANT)
                    && !self.is_behind_a_reference(&ast::Pat::TupleStructPat(pattern.clone())))
                .then(|| {
                    self.semantics
                        .type_of_pat(&ast::Pat::TupleStructPat(pattern))
                })
                .flatten()
                .and_then(|TypeInfo { original, .. }| self.error_types.result_error(&original))
                .into_iter()
                .collect()
            })
            .or_else(|| {
                ast::TuplePat::cast(parent.clone()).map(|pattern| {
                    let written = pattern.fields().count();
                    let before = pattern
                        .fields()
                        .take_while(|field| !matches!(field, ast::Pat::RestPat(_)))
                        .count();
                    let after = written.saturating_sub(before).saturating_sub(1);

                    (!self.is_behind_a_reference(&ast::Pat::TuplePat(pattern.clone())))
                        .then(|| self.semantics.type_of_pat(&ast::Pat::TuplePat(pattern)))
                        .flatten()
                        .map(|TypeInfo { original, .. }| {
                            let elements = original.tuple_fields(database);
                            let omitted =
                                elements.len().saturating_sub(before).saturating_sub(after);

                            elements.into_iter().skip(before).take(omitted).collect()
                        })
                        .unwrap_or_default()
                })
            })
            .or_else(|| {
                ast::RecordPatFieldList::cast(parent.clone())
                    .and_then(|fields| fields.syntax().parent())
                    .and_then(ast::RecordPat::cast)
                    .filter(|pattern| {
                        !self.is_behind_a_reference(&ast::Pat::RecordPat(pattern.clone()))
                    })
                    .map(|pattern| {
                        self.semantics
                            .record_pattern_matched_fields(&pattern)
                            .into_iter()
                            .map(|(_, omitted)| omitted)
                            .collect()
                    })
            })
            .unwrap_or_default()
    }

    fn owns_a_result_error(&self, expression: &ast::Expr) -> bool {
        self.semantics
            .type_of_expr(expression)
            .is_some_and(|TypeInfo { original, .. }| {
                self.error_types.owned_result_error(&original).is_some()
            })
    }

    fn rest(&self, rest: &ast::RestPat) -> Option<SyntaxNode> {
        rest.syntax()
            .parent()
            .filter(|parent| {
                self.omitted_by(parent)
                    .iter()
                    .any(|element| self.error_types.is_discardable(element))
            })
            .map(|_| rest.syntax().clone())
    }

    fn underscore(&self, underscore: &ast::UnderscoreExpr) -> Option<SyntaxNode> {
        self.semantics
            .type_of_expr(&ast::Expr::UnderscoreExpr(underscore.clone()))
            .filter(|TypeInfo { original, .. }| self.error_types.is_discardable(original))
            .map(|_| underscore.syntax().clone())
    }

    fn unread(&self, binding: &ast::IdentPat) -> Option<SyntaxNode> {
        (self.semantics.resolve_bind_pat_to_const(binding).is_none()
            && !is_bodiless_parameter(binding.syntax()))
        .then(|| self.semantics.type_of_binding_in_pat(binding))
        .flatten()
        .filter(|bound| self.error_types.is_discardable(bound))
        .and_then(|_| self.semantics.to_def(binding))
        .filter(|local| {
            !Definition::Local(*local)
                .usages(self.semantics)
                .all()
                .references
                .values()
                .flatten()
                .any(|FileReference { category, .. }| category.contains(ReferenceCategory::READ))
        })
        .and_then(|_| binding.name())
        .map(|name| name.syntax().clone())
    }

    fn value_only(&self, pattern: &ast::TupleStructPat) -> Option<SyntaxNode> {
        let top = pattern
            .syntax()
            .parent()
            .and_then(ast::OrPat::cast)
            .map_or_else(
                || pattern.syntax().clone(),
                |alternatives| alternatives.syntax().clone(),
            );
        let binds_the_error = ast::OrPat::cast(top.clone()).is_some_and(|alternatives| {
            alternatives.pats().any(|alternative| {
                ast::TupleStructPat::cast(alternative.syntax().clone()).is_some_and(|alternative| {
                    self.is_result_variant(&alternative, RESULT_ERROR_VARIANT)
                })
            })
        });
        let refutably_matched = top.parent().is_some_and(|owner| {
            ast::LetExpr::can_cast(owner.kind())
                || ast::LetStmt::cast(owner).is_some_and(|statement| statement.let_else().is_some())
        });

        (self.is_result_variant(pattern, RESULT_VALUE_VARIANT)
            && !binds_the_error
            && refutably_matched
            && !self.is_behind_a_reference(&ast::Pat::TupleStructPat(pattern.clone())))
        .then(|| {
            self.semantics
                .type_of_pat(&ast::Pat::TupleStructPat(pattern.clone()))
        })
        .flatten()
        .filter(|TypeInfo { original, .. }| self.error_types.owned_result_error(original).is_some())
        .map(|_| pattern.syntax().clone())
    }

    fn wildcard(&self, wildcard: &ast::Pat) -> Option<SyntaxNode> {
        (!self.is_behind_a_reference(wildcard) && !is_bodiless_parameter(wildcard.syntax()))
            .then(|| self.semantics.type_of_pat(wildcard))
            .flatten()
            .filter(|TypeInfo { original, .. }| self.error_types.is_discardable(original))
            .map(|_| wildcard.syntax().clone())
    }
}

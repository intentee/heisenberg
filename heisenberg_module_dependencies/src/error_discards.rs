use std::iter;
use std::sync::Arc;

use ra_ap_hir::Access;
use ra_ap_hir::Adt;
use ra_ap_hir::AsAssocItem;
use ra_ap_hir::AssocItemContainer;
use ra_ap_hir::Function;
use ra_ap_hir::InFile;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_hir::Type;
use ra_ap_hir::TypeInfo;
use ra_ap_hir::TypeParam;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_ide_db::search::FileReference;
use ra_ap_ide_db::search::ReferenceCategory;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasArgList;
use ra_ap_syntax::ast::HasGenericParams;
use ra_ap_syntax::ast::HasName;
use ra_ap_syntax::ast::HasTypeBounds;

use heisenberg_task_graph::once_map::OnceMap;

use crate::constants::RESULT_ERROR_VARIANT;
use crate::constants::RESULT_VALUE_VARIANT;
use crate::error_types::ErrorTypes;
use crate::is_trait_item::is_trait_item;
use crate::method_call_resolutions::MethodCallResolutions;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::path_resolutions::PathResolutions;
use crate::result_method_effect::ResultMethodEffect;
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

fn omitted_elements(
    fields: impl Iterator<Item = ast::Pat> + Clone,
    elements: Vec<Type<'_>>,
) -> Vec<Type<'_>> {
    let before = fields
        .clone()
        .take_while(|field| !matches!(field, ast::Pat::RestPat(_)))
        .count();
    let after = fields
        .skip_while(|field| !matches!(field, ast::Pat::RestPat(_)))
        .skip(1)
        .count();

    elements
        .into_iter()
        .skip(before)
        .rev()
        .skip(after)
        .rev()
        .collect()
}

pub struct ErrorDiscards<'discards, 'database> {
    pub error_types: &'discards ErrorTypes<'discards, 'database>,
    pub method_calls: &'discards MethodCallResolutions<'discards, 'database>,
    pub paths: &'discards PathResolutions<'discards, 'database>,
    pub result_method_effects:
        &'discards OnceMap<Function, Result<ResultMethodEffect, Arc<ModuleDependenciesError>>>,
    pub semantics: &'discards Semantics<'database, RootDatabase>,
    pub standard_items: &'discards StandardItems,
}

impl<'database> ErrorDiscards<'_, 'database> {
    /// # Errors
    ///
    /// Returns [`ModuleDependenciesError::ResultMethodWithoutSource`] when rust-analyzer gives
    /// no source for a method of `Result`, whose signature decides whether it drops the error.
    pub fn discarded(
        &self,
        node: &SyntaxNode,
    ) -> Result<Option<SyntaxNode>, ModuleDependenciesError> {
        ast::MethodCallExpr::cast(node.clone()).map_or_else(
            || {
                ast::PathExpr::cast(node.clone()).map_or_else(
                    || {
                        Ok(ast::WildcardPat::cast(node.clone())
                            .and_then(|wildcard| self.wildcard(&ast::Pat::WildcardPat(wildcard)))
                            .or_else(|| {
                                ast::RestPat::cast(node.clone()).and_then(|rest| self.rest(&rest))
                            })
                            .or_else(|| {
                                ast::UnderscoreExpr::cast(node.clone())
                                    .and_then(|underscore| self.underscore(&underscore))
                            })
                            .or_else(|| {
                                ast::TupleStructPat::cast(node.clone())
                                    .and_then(|pattern| self.value_only(&pattern))
                            })
                            .or_else(|| {
                                ast::ForExpr::cast(node.clone())
                                    .and_then(|repetition| self.iterated(&repetition))
                            })
                            .or_else(|| {
                                ast::IdentPat::cast(node.clone())
                                    .and_then(|binding| self.unread(&binding))
                            }))
                    },
                    |path| self.function_value(&path),
                )
            },
            |call| self.method_call(&call),
        )
    }

    fn classified(
        &self,
        function: Function,
    ) -> Result<ResultMethodEffect, ModuleDependenciesError> {
        let database = self.semantics.db;

        match function
            .as_assoc_item(database)
            .map(|item| item.container(database))
        {
            Some(AssocItemContainer::Impl(implementation)) => {
                let implemented = implementation.self_ty(database);
                let declaring = implementation.trait_(database);

                if implemented.as_adt() != Some(Adt::Enum(self.standard_items.result)) {
                    Ok(ResultMethodEffect::NotAResultMethod)
                } else if declaring == Some(self.standard_items.into_iterator) {
                    Ok(ResultMethodEffect::DropsTheError)
                } else if declaring.is_some() {
                    Ok(ResultMethodEffect::NotAResultMethod)
                } else {
                    function
                        .self_param(database)
                        .and_then(|parameter| {
                            implemented
                                .type_arguments()
                                .nth(1)
                                .and_then(|error| error.as_type_param(database))
                                .map(|error| {
                                    self.names_the_error(function, error).map(|names| {
                                        match parameter.access(database) {
                                            Access::Owned if names => {
                                                ResultMethodEffect::KeepsTheError
                                            }
                                            Access::Owned => ResultMethodEffect::DropsTheError,
                                            Access::Shared | Access::Exclusive if names => {
                                                ResultMethodEffect::LendsTheError
                                            }
                                            Access::Shared | Access::Exclusive => {
                                                ResultMethodEffect::DropsATemporaryError
                                            }
                                        }
                                    })
                                })
                        })
                        .unwrap_or(Ok(ResultMethodEffect::NotAResultMethod))
                }
            }
            _ => Ok(ResultMethodEffect::NotAResultMethod),
        }
    }

    fn drops_through(
        &self,
        function: Function,
        call: &ast::MethodCallExpr,
        receiver: &ast::Expr,
    ) -> Result<bool, ModuleDependenciesError> {
        let database = self.semantics.db;

        self.effect(function).and_then(|effect| match effect {
            ResultMethodEffect::DropsTheError => {
                if self.owns_a_result_error(receiver) {
                    Ok(true)
                } else {
                    self.lender(receiver.clone()).map(|lender| {
                        lender.syntax() != receiver.syntax() && self.is_owned_temporary(&lender)
                    })
                }
            }
            ResultMethodEffect::DropsATemporaryError => self
                .lender(receiver.clone())
                .map(|lender| self.is_owned_temporary(&lender)),
            ResultMethodEffect::KeepsTheError | ResultMethodEffect::LendsTheError => Ok(false),
            ResultMethodEffect::NotAResultMethod => Ok((self
                .is_iterator_method(function, "flatten")
                && self
                    .semantics
                    .type_of_expr(receiver)
                    .and_then(|TypeInfo { original, .. }| original.iterator_item(database))
                    .is_some_and(|item| self.error_types.owned_result_error(&item).is_some()))
                || (self.is_iterator_method(function, "flat_map")
                    && call
                        .arg_list()
                        .and_then(|arguments| arguments.args().next())
                        .and_then(|mapping| self.semantics.type_of_expr(&mapping))
                        .and_then(|TypeInfo { original, .. }| original.as_callable(database))
                        .is_some_and(|mapping| {
                            self.error_types
                                .owned_result_error(&mapping.return_type())
                                .is_some()
                        }))),
        })
    }

    fn effect(&self, function: Function) -> Result<ResultMethodEffect, ModuleDependenciesError> {
        self.result_method_effects
            .get_or_init(function, || self.classified(function).map_err(Arc::new))
            .map_err(ModuleDependenciesError::ResultMethodUnavailable)
    }

    fn function_value(
        &self,
        path: &ast::PathExpr,
    ) -> Result<Option<SyntaxNode>, ModuleDependenciesError> {
        let database = self.semantics.db;

        path.path()
            .and_then(|written| match self.paths.resolve(&written) {
                Some(PathResolution::Def(ModuleDef::Function(function))) => Some(function),
                _ => None,
            })
            .map(|function| {
                if function == self.standard_items.drop {
                    Ok(true)
                } else {
                    self.effect(function)
                        .map(|effect| effect == ResultMethodEffect::DropsTheError)
                }
            })
            .transpose()
            .map(|drops| {
                drops
                    .filter(|drops| *drops)
                    .and_then(|_| {
                        self.semantics
                            .type_of_expr(&ast::Expr::PathExpr(path.clone()))
                    })
                    .and_then(|TypeInfo { original, .. }| original.as_callable(database))
                    .and_then(|callable| callable.params().into_iter().next())
                    .filter(|parameter| self.error_types.is_discardable(parameter.ty()))
                    .map(|_| path.syntax().clone())
            })
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

    fn is_drop_bound(&self, predicate: &ast::WherePred) -> bool {
        predicate.type_bound_list().is_some_and(|bounds| {
            bounds.bounds().all(|bound| {
                bound
                    .ty()
                    .and_then(|bounding| ast::PathType::cast(bounding.syntax().clone()))
                    .and_then(|written| written.path())
                    .is_some_and(|path| {
                        matches!(
                            self.semantics.resolve_path(&path),
                            Some(PathResolution::Def(ModuleDef::Trait(bounding)))
                                if bounding == self.standard_items.destruct
                        )
                    })
            })
        })
    }

    fn is_iterator_method(&self, function: Function, name: &str) -> bool {
        let database = self.semantics.db;

        function.name(database).as_str() == name
            && is_trait_item(database, function, self.standard_items.iterator)
    }

    fn is_owned_temporary(&self, expression: &ast::Expr) -> bool {
        !self.is_place(expression) && self.owns_a_result_error(expression)
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

    fn lender(&self, expression: ast::Expr) -> Result<ast::Expr, ModuleDependenciesError> {
        ast::MethodCallExpr::cast(expression.syntax().clone())
            .and_then(|call| {
                self.method_calls.resolve(&call).map(
                    |SubstitutedResolution {
                         resolution: function,
                         ..
                     }| {
                        self.passes_the_borrow_on(function, &call)
                            .map(|passes| passes.then(|| call.receiver()).flatten())
                    },
                )
            })
            .transpose()
            .map(Option::flatten)
            .and_then(|receiver| receiver.map_or(Ok(expression), |receiver| self.lender(receiver)))
    }

    fn method_call(
        &self,
        call: &ast::MethodCallExpr,
    ) -> Result<Option<SyntaxNode>, ModuleDependenciesError> {
        self.method_calls
            .resolve(call)
            .and_then(
                |SubstitutedResolution {
                     resolution: function,
                     ..
                 }| {
                    call.receiver()
                        .map(|receiver| self.drops_through(function, call, &receiver))
                },
            )
            .transpose()
            .map(|drops| {
                drops
                    .filter(|drops| *drops)
                    .and_then(|_| call.name_ref())
                    .map(|name| name.syntax().clone())
            })
    }

    fn names_the_error(
        &self,
        function: Function,
        error: TypeParam,
    ) -> Result<bool, ModuleDependenciesError> {
        self.semantics
            .source(function)
            .ok_or(ModuleDependenciesError::ResultMethodWithoutSource(function))
            .map(
                |InFile {
                     value: declared, ..
                 }| {
                    declared
                        .generic_param_list()
                        .map(|parameters| parameters.syntax().clone())
                        .into_iter()
                        .chain(
                            declared
                                .param_list()
                                .map(|parameters| parameters.syntax().clone()),
                        )
                        .chain(
                            declared
                                .ret_type()
                                .map(|returned| returned.syntax().clone()),
                        )
                        .chain(
                            declared
                                .where_clause()
                                .into_iter()
                                .flat_map(|clause| clause.predicates())
                                .filter(|predicate| !self.is_drop_bound(predicate))
                                .map(|predicate| predicate.syntax().clone()),
                        )
                        .flat_map(|signature| signature.descendants())
                        .filter_map(ast::Path::cast)
                        .any(|written| match self.semantics.resolve_path(&written) {
                            Some(PathResolution::TypeParam(named)) => named == error,
                            Some(PathResolution::SelfType(_)) => true,
                            _ => false,
                        })
                },
            )
    }

    fn omitted_by(&self, parent: &SyntaxNode) -> Vec<Type<'database>> {
        let database = self.semantics.db;

        ast::TupleStructPat::cast(parent.clone())
            .map(|pattern| {
                (!self.is_behind_a_reference(&ast::Pat::TupleStructPat(pattern.clone())))
                    .then(|| self.semantics.resolve_tuple_struct_pat_fields(&pattern))
                    .flatten()
                    .map(|fields| {
                        omitted_elements(
                            pattern.fields(),
                            fields.into_iter().map(|(_, field)| field).collect(),
                        )
                    })
                    .unwrap_or_default()
            })
            .or_else(|| {
                ast::TuplePat::cast(parent.clone()).map(|pattern| {
                    (!self.is_behind_a_reference(&ast::Pat::TuplePat(pattern.clone())))
                        .then(|| {
                            self.semantics
                                .type_of_pat(&ast::Pat::TuplePat(pattern.clone()))
                        })
                        .flatten()
                        .map(|TypeInfo { original, .. }| {
                            omitted_elements(pattern.fields(), original.tuple_fields(database))
                        })
                        .unwrap_or_default()
                })
            })
            .or_else(|| {
                ast::SlicePat::cast(parent.clone()).map(|pattern| {
                    (!self.is_behind_a_reference(&ast::Pat::SlicePat(pattern.clone())))
                        .then(|| {
                            self.semantics
                                .type_of_pat(&ast::Pat::SlicePat(pattern.clone()))
                        })
                        .flatten()
                        .and_then(|TypeInfo { original, .. }| original.as_array(database))
                        .map(|(element, length)| {
                            omitted_elements(
                                pattern.pats(),
                                iter::repeat_n(element, length).collect(),
                            )
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

    fn passes_the_borrow_on(
        &self,
        function: Function,
        call: &ast::MethodCallExpr,
    ) -> Result<bool, ModuleDependenciesError> {
        self.effect(function).map(|effect| match effect {
            ResultMethodEffect::LendsTheError => true,
            ResultMethodEffect::KeepsTheError => self
                .semantics
                .type_of_expr(&ast::Expr::MethodCallExpr(call.clone()))
                .and_then(|TypeInfo { original, .. }| self.error_types.result_error(&original))
                .is_some_and(|error| error.is_reference()),
            ResultMethodEffect::DropsATemporaryError
            | ResultMethodEffect::DropsTheError
            | ResultMethodEffect::NotAResultMethod => false,
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

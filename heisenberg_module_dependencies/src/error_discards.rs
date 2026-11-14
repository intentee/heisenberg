use std::iter;
use std::sync::Arc;

use ra_ap_hir::Access;
use ra_ap_hir::Adjust;
use ra_ap_hir::Adjustment;
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

fn omitted_elements<'database>(
    fields: impl Iterator<Item = ast::Pat> + Clone,
    elements: impl DoubleEndedIterator<Item = Type<'database>> + ExactSizeIterator,
) -> impl Iterator<Item = Type<'database>> {
    let before = fields
        .clone()
        .take_while(|field| !matches!(field, ast::Pat::RestPat(_)))
        .count();
    let after = fields
        .skip_while(|field| !matches!(field, ast::Pat::RestPat(_)))
        .skip(1)
        .count();

    elements.skip(before).rev().skip(after)
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
    /// Returns [`ModuleDependenciesError::ResultMethodUnavailable`], which holds
    /// [`ModuleDependenciesError::ResultMethodWithoutSource`], when rust-analyzer gives no source
    /// for a method of `Result`, whose signature decides whether it drops the error.
    pub fn discarded(
        &self,
        node: &SyntaxNode,
    ) -> Result<Option<SyntaxNode>, ModuleDependenciesError> {
        ast::MethodCallExpr::cast(node.clone())
            .map(|call| self.method_call(&call))
            .or_else(|| ast::PathExpr::cast(node.clone()).map(|path| self.function_value(&path)))
            .or_else(|| {
                ast::ForExpr::cast(node.clone()).map(|repetition| self.iterated(&repetition))
            })
            .or_else(|| {
                ast::WildcardPat::cast(node.clone())
                    .map(|wildcard| self.wildcard(&ast::Pat::WildcardPat(wildcard)))
            })
            .or_else(|| {
                ast::UnderscoreExpr::cast(node.clone())
                    .map(|underscore| self.underscore(&underscore))
            })
            .or_else(|| {
                ast::TupleStructPat::cast(node.clone()).map(|pattern| self.value_only(&pattern))
            })
            .or_else(|| ast::RestPat::cast(node.clone()).map(|rest| self.rest(&rest)))
            .or_else(|| ast::IdentPat::cast(node.clone()).map(|binding| self.unread(&binding)))
            .unwrap_or(Ok(None))
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
                let declaring = implementation.trait_(database);

                let implemented = implementation.self_ty(database);

                match self.error_types.result_error(&implemented) {
                    None if declaring == Some(self.standard_items.into_iterator)
                        && implemented.is_reference()
                        && self
                            .error_types
                            .result_error(&implemented.strip_references())
                            .is_some() =>
                    {
                        Ok(ResultMethodEffect::DropsATemporaryError)
                    }
                    None => Ok(ResultMethodEffect::NotAResultMethod),
                    Some(_) if declaring == Some(self.standard_items.into_iterator) => {
                        Ok(ResultMethodEffect::DropsTheError)
                    }
                    Some(_) if declaring.is_some() => Ok(ResultMethodEffect::NotAResultMethod),
                    Some(error) => function
                        .self_param(database)
                        .and_then(|parameter| {
                            error.as_type_param(database).map(|error| {
                                self.names_the_error(function, error).map(|names| {
                                    match parameter.access(database) {
                                        Access::Owned if names => ResultMethodEffect::KeepsTheError,
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
                        .unwrap_or(Ok(ResultMethodEffect::NotAResultMethod)),
                }
            }
            _ => Ok(ResultMethodEffect::NotAResultMethod),
        }
    }

    fn discards(
        &self,
        held: &Type<'database>,
        behind_a_reference: bool,
        holder: &SyntaxNode,
    ) -> Result<bool, ModuleDependenciesError> {
        if !behind_a_reference && self.error_types.is_discardable(held) {
            Ok(true)
        } else if self.holds_an_error(held) {
            self.lent_by_a_temporary(holder)
        } else {
            Ok(false)
        }
    }

    fn drops_an_error_of(&self, expression: &ast::Expr) -> Result<bool, ModuleDependenciesError> {
        if self.owns_a_result_error(expression) {
            Ok(true)
        } else {
            self.lends_a_temporary(expression)
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
            ResultMethodEffect::DropsTheError => self.drops_an_error_of(receiver),
            ResultMethodEffect::DropsATemporaryError => self
                .lender(receiver)
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

    fn element_value(
        &self,
        element: &SyntaxNode,
        mut elements: impl Iterator<Item = SyntaxNode>,
        container: &SyntaxNode,
    ) -> Option<ast::Expr> {
        let position = elements.position(|candidate| candidate == *element);

        self.lending_value(container).map(|value| {
            ast::TupleExpr::cast(value.syntax().clone())
                .and_then(|values| position.and_then(|position| values.fields().nth(position)))
                .unwrap_or(value)
        })
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
                .and_then(|drops| {
                    let parameter_discardable = drops
                        && self
                            .semantics
                            .type_of_expr(&ast::Expr::PathExpr(path.clone()))
                            .and_then(|TypeInfo { original, .. }| original.as_callable(database))
                            .and_then(|callable| callable.params().into_iter().next())
                            .is_some_and(|parameter| {
                                self.error_types.is_discardable(parameter.ty())
                            });

                    if parameter_discardable || function != self.standard_items.drop {
                        Ok(parameter_discardable)
                    } else {
                        path.syntax()
                            .parent()
                            .and_then(ast::CallExpr::cast)
                            .and_then(|call| call.arg_list())
                            .and_then(|arguments| arguments.args().next())
                            .map_or(Ok(false), |argument| self.lends_a_temporary(&argument))
                    }
                })
            })
            .transpose()
            .map(|drops| drops.filter(|drops| *drops).map(|_| path.syntax().clone()))
    }

    fn holds_an_error(&self, candidate: &Type<'database>) -> bool {
        let held = candidate.strip_references();

        self.error_types.is_discardable(&held)
            || self
                .error_types
                .result_error(&held)
                .is_some_and(|error| self.error_types.implements_error(&error.strip_references()))
    }

    fn is_behind_a_pointer(&self, owner: &ast::Expr) -> bool {
        self.semantics
            .type_of_expr(owner)
            .is_some_and(|TypeInfo { original, .. }| {
                original.is_reference() || original.is_raw_ptr()
            })
            || self
                .semantics
                .expr_adjustments(owner)
                .is_some_and(|adjustments| {
                    adjustments
                        .iter()
                        .any(|Adjustment { kind, .. }| matches!(kind, Adjust::Deref(_)))
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
            ast::Expr::FieldExpr(field) => field
                .expr()
                .is_some_and(|owner| self.is_place(&owner) || self.is_behind_a_pointer(&owner)),
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

    fn iterated(
        &self,
        repetition: &ast::ForExpr,
    ) -> Result<Option<SyntaxNode>, ModuleDependenciesError> {
        repetition
            .iterable()
            .map(|iterable| {
                self.drops_an_error_of(&iterable)
                    .map(|drops| drops.then(|| iterable.syntax().clone()))
            })
            .transpose()
            .map(Option::flatten)
    }

    fn lender(&self, expression: &ast::Expr) -> Result<ast::Expr, ModuleDependenciesError> {
        self.next_lender(expression)
            .and_then(|next| next.map_or_else(|| Ok(expression.clone()), |next| self.lender(&next)))
    }

    fn lending_value(&self, node: &SyntaxNode) -> Option<ast::Expr> {
        node.parent().and_then(|parent| {
            ast::TuplePat::cast(parent.clone())
                .map(|tuple| {
                    self.element_value(
                        node,
                        tuple.fields().map(|field| field.syntax().clone()),
                        &parent,
                    )
                })
                .or_else(|| {
                    ast::TupleExpr::cast(parent.clone()).map(|tuple| {
                        self.element_value(
                            node,
                            tuple.fields().map(|field| field.syntax().clone()),
                            &parent,
                        )
                    })
                })
                .unwrap_or_else(|| {
                    if ast::Pat::can_cast(parent.kind())
                        || ast::RecordPatField::can_cast(parent.kind())
                        || ast::RecordPatFieldList::can_cast(parent.kind())
                        || ast::MatchArm::can_cast(parent.kind())
                        || ast::MatchArmList::can_cast(parent.kind())
                        || ast::Param::can_cast(parent.kind())
                        || ast::ParamList::can_cast(parent.kind())
                    {
                        self.lending_value(&parent)
                    } else {
                        ast::MatchExpr::cast(parent.clone())
                            .and_then(|matched| matched.expr())
                            .or_else(|| {
                                ast::LetStmt::cast(parent.clone())
                                    .and_then(|statement| statement.initializer())
                            })
                            .or_else(|| {
                                ast::LetExpr::cast(parent.clone())
                                    .and_then(|condition| condition.expr())
                            })
                            .or_else(|| {
                                ast::BinExpr::cast(parent.clone())
                                    .filter(|assignment| {
                                        assignment
                                            .lhs()
                                            .is_some_and(|assignee| assignee.syntax() == node)
                                    })
                                    .and_then(|assignment| assignment.rhs())
                            })
                            .or_else(|| {
                                ast::ClosureExpr::cast(parent.clone())
                                    .and_then(|closure| closure.syntax().parent())
                                    .and_then(ast::ArgList::cast)
                                    .and_then(|arguments| arguments.syntax().parent())
                                    .and_then(ast::MethodCallExpr::cast)
                                    .and_then(|call| call.receiver())
                            })
                    }
                })
        })
    }

    fn lends_a_temporary(&self, expression: &ast::Expr) -> Result<bool, ModuleDependenciesError> {
        self.lender(expression).map(|lender| {
            lender.syntax() != expression.syntax() && self.is_owned_temporary(&lender)
        })
    }

    fn lent_by_a_temporary(&self, holder: &SyntaxNode) -> Result<bool, ModuleDependenciesError> {
        self.lending_value(holder)
            .map_or(Ok(false), |value| self.lends_a_temporary(&value))
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

    fn next_lender(
        &self,
        expression: &ast::Expr,
    ) -> Result<Option<ast::Expr>, ModuleDependenciesError> {
        match expression {
            ast::Expr::ParenExpr(inner) => Ok(inner.expr()),
            ast::Expr::RefExpr(borrow) => Ok(borrow.expr()),
            _ => ast::MethodCallExpr::cast(expression.syntax().clone())
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
                .map(Option::flatten),
        }
    }

    fn omitted_by(&self, parent: &SyntaxNode) -> Vec<Type<'database>> {
        let database = self.semantics.db;

        ast::TupleStructPat::cast(parent.clone())
            .map(|pattern| {
                self.semantics
                    .resolve_tuple_struct_pat_fields(&pattern)
                    .map(|fields| {
                        omitted_elements(
                            pattern.fields(),
                            fields.into_iter().map(|(_, field)| field),
                        )
                        .collect()
                    })
                    .unwrap_or_default()
            })
            .or_else(|| {
                ast::TuplePat::cast(parent.clone()).map(|pattern| {
                    self.semantics
                        .type_of_pat(&ast::Pat::TuplePat(pattern.clone()))
                        .map(|TypeInfo { original, .. }| {
                            omitted_elements(
                                pattern.fields(),
                                original.tuple_fields(database).into_iter(),
                            )
                            .collect()
                        })
                        .unwrap_or_default()
                })
            })
            .or_else(|| {
                ast::SlicePat::cast(parent.clone()).map(|pattern| {
                    self.semantics
                        .type_of_pat(&ast::Pat::SlicePat(pattern.clone()))
                        .and_then(|TypeInfo { original, .. }| original.as_array(database))
                        .map(|(element, length)| {
                            omitted_elements(pattern.pats(), iter::repeat_n(element, length))
                                .take(1)
                                .collect()
                        })
                        .unwrap_or_default()
                })
            })
            .or_else(|| {
                ast::RecordPatFieldList::cast(parent.clone())
                    .and_then(|fields| fields.syntax().parent())
                    .and_then(ast::RecordPat::cast)
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
                .is_some_and(|TypeInfo { original, .. }| {
                    (original.is_reference()
                        && self
                            .error_types
                            .implements_error(&original.strip_references()))
                        || self
                            .error_types
                            .result_error(&original)
                            .is_some_and(|error| error.is_reference())
                }),
            ResultMethodEffect::DropsATemporaryError
            | ResultMethodEffect::DropsTheError
            | ResultMethodEffect::NotAResultMethod => false,
        })
    }

    fn rest(&self, rest: &ast::RestPat) -> Result<Option<SyntaxNode>, ModuleDependenciesError> {
        rest.syntax()
            .parent()
            .and_then(|parent| {
                ast::Pat::cast(parent.clone())
                    .or_else(|| parent.parent().and_then(ast::Pat::cast))
                    .map(|owner| {
                        let omitted = self.omitted_by(&parent);

                        if !self.is_behind_a_reference(&owner)
                            && omitted
                                .iter()
                                .any(|element| self.error_types.is_discardable(element))
                        {
                            Ok(true)
                        } else if omitted.iter().any(|element| self.holds_an_error(element)) {
                            self.lent_by_a_temporary(owner.syntax())
                        } else {
                            Ok(false)
                        }
                    })
            })
            .transpose()
            .map(|drops| drops.filter(|drops| *drops).map(|_| rest.syntax().clone()))
    }

    fn underscore(
        &self,
        underscore: &ast::UnderscoreExpr,
    ) -> Result<Option<SyntaxNode>, ModuleDependenciesError> {
        self.semantics
            .type_of_expr(&ast::Expr::UnderscoreExpr(underscore.clone()))
            .map(|TypeInfo { original, .. }| self.discards(&original, false, underscore.syntax()))
            .transpose()
            .map(|drops| {
                drops
                    .filter(|drops| *drops)
                    .map(|_| underscore.syntax().clone())
            })
    }

    fn unread(
        &self,
        binding: &ast::IdentPat,
    ) -> Result<Option<SyntaxNode>, ModuleDependenciesError> {
        (self.semantics.resolve_bind_pat_to_const(binding).is_none()
            && !is_bodiless_parameter(binding.syntax()))
        .then(|| self.semantics.type_of_binding_in_pat(binding))
        .flatten()
        .map(|bound| self.discards(&bound, false, binding.syntax()))
        .transpose()
        .map(|holds| {
            holds
                .filter(|holds| *holds)
                .and_then(|_| self.semantics.to_def(binding))
                .filter(|local| {
                    !Definition::Local(*local)
                        .usages(self.semantics)
                        .all()
                        .references
                        .values()
                        .flatten()
                        .any(|FileReference { category, .. }| {
                            category.contains(ReferenceCategory::READ)
                        })
                })
                .and_then(|_| binding.name())
                .map(|name| name.syntax().clone())
        })
    }

    fn value_only(
        &self,
        pattern: &ast::TupleStructPat,
    ) -> Result<Option<SyntaxNode>, ModuleDependenciesError> {
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
            && refutably_matched)
            .then(|| {
                if !self.is_behind_a_reference(&ast::Pat::TupleStructPat(pattern.clone()))
                    && self
                        .semantics
                        .type_of_pat(&ast::Pat::TupleStructPat(pattern.clone()))
                        .is_some_and(|TypeInfo { original, .. }| {
                            self.error_types.owned_result_error(&original).is_some()
                        })
                {
                    Ok(true)
                } else {
                    self.lent_by_a_temporary(pattern.syntax())
                }
            })
            .transpose()
            .map(|drops| {
                drops
                    .filter(|drops| *drops)
                    .map(|_| pattern.syntax().clone())
            })
    }

    fn wildcard(&self, wildcard: &ast::Pat) -> Result<Option<SyntaxNode>, ModuleDependenciesError> {
        (!is_bodiless_parameter(wildcard.syntax()))
            .then(|| self.semantics.type_of_pat(wildcard))
            .flatten()
            .map(|TypeInfo { original, .. }| {
                self.discards(
                    &original,
                    self.is_behind_a_reference(wildcard),
                    wildcard.syntax(),
                )
            })
            .transpose()
            .map(|drops| {
                drops
                    .filter(|drops| *drops)
                    .map(|_| wildcard.syntax().clone())
            })
    }
}

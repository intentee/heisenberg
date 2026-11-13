use std::collections::HashSet;
use std::iter;

use ra_ap_hir::AsAssocItem;
use ra_ap_hir::AssocItemContainer;
use ra_ap_hir::Function;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Local;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_hir::Trait;
use ra_ap_hir::Type;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasArgList;

use crate::constants::ROUND_TRIP_TRANSPARENT_METHODS;
use crate::constants::TEXT_DECODING_METHOD;
use crate::method_call_resolutions::MethodCallResolutions;
use crate::path_resolutions::PathResolutions;
use crate::serialization_traits::SerializationTraits;
use crate::standard_items::StandardItems;
use crate::substituted_resolution::SubstitutedResolution;
use crate::trace::Trace;

pub struct RoundTrips<'trips, 'database> {
    pub method_calls: &'trips MethodCallResolutions<'trips, 'database>,
    pub paths: &'trips PathResolutions<'trips, 'database>,
    pub semantics: &'trips Semantics<'database, RootDatabase>,
    pub serialization_traits: &'trips SerializationTraits,
    pub standard_items: &'trips StandardItems,
}

impl<'database> RoundTrips<'_, 'database> {
    pub fn compares_a_round_trip(&self, call: &ast::MacroCall, body: &SyntaxNode) -> bool {
        self.semantics.to_def(call).is_some_and(|expansion| {
            self.operands(&self.semantics.parse_or_expand(HirFileId::from(expansion)))
                .is_some_and(|operands| {
                    let traces: Vec<Trace<'database>> = operands
                        .fields()
                        .filter_map(|operand| self.trace(operand, body))
                        .collect();

                    traces.iter().any(|trace| match trace {
                        Trace::Decoded(value) => traces.contains(&Trace::Root(*value)),
                        Trace::Encoded(_) | Trace::Root(_) => false,
                    })
                })
        })
    }

    fn bound(&self, local: Local<'database>, body: &SyntaxNode) -> Trace<'database> {
        body.descendants()
            .filter_map(ast::IdentPat::cast)
            .find(|binding| self.semantics.to_def(binding) == Some(local))
            .and_then(|binding| binding.syntax().parent().and_then(ast::LetStmt::cast))
            .and_then(|statement| statement.initializer())
            .and_then(|initializer| self.trace(initializer, body))
            .unwrap_or(Trace::Root(local))
    }

    fn called(&self, call: &ast::CallExpr, body: &SyntaxNode) -> Option<Trace<'database>> {
        let database = self.semantics.db;
        let arguments: Vec<ast::Expr> = call
            .arg_list()
            .map(|list| list.args().collect())
            .unwrap_or_default();

        match call
            .expr()
            .and_then(|callee| ast::PathExpr::cast(callee.syntax().clone()))
            .and_then(|callee| callee.path())
            .and_then(|written| self.paths.resolve(&written))
        {
            Some(PathResolution::Def(ModuleDef::Function(function))) => {
                if self.is_trait_method(function, self.standard_items.from_str)
                    || self.returns_a_deserialized_value(function)
                {
                    arguments
                        .into_iter()
                        .find_map(|argument| self.trace(argument, body).and_then(Trace::decoded))
                } else {
                    function
                        .params_without_self(database)
                        .iter()
                        .position(|parameter| self.is_serialized(parameter.ty()))
                        .and_then(|position| arguments.into_iter().nth(position))
                        .and_then(|argument| self.trace(argument, body))
                        .and_then(Trace::encoded)
                }
            }
            _ => None,
        }
    }

    fn is_bounded_by(&self, candidate: &Type<'database>, traits: &HashSet<Trait>) -> bool {
        candidate
            .strip_references()
            .as_type_param(self.semantics.db)
            .is_some_and(|parameter| {
                parameter
                    .trait_bounds(self.semantics.db)
                    .iter()
                    .any(|bound| traits.contains(bound))
            })
    }

    fn is_serialized(&self, parameter: &Type<'database>) -> bool {
        self.is_bounded_by(parameter, &self.serialization_traits.serialize)
    }

    fn is_standard(&self, method: Function) -> bool {
        let database = self.semantics.db;

        self.standard_items
            .library_crates
            .contains(&method.module(database).krate(database))
    }

    fn is_text_decoding(&self, method: Function) -> bool {
        let database = self.semantics.db;

        method.name(database).as_str() == TEXT_DECODING_METHOD
            && self.is_standard(method)
            && matches!(
                method.as_assoc_item(database).map(|item| item.container(database)),
                Some(AssocItemContainer::Impl(implementation))
                    if implementation.self_ty(database).is_str()
            )
    }

    fn is_trait_method(&self, method: Function, owner: Trait) -> bool {
        let database = self.semantics.db;

        method
            .as_assoc_item(database)
            .and_then(|item| item.container_or_implemented_trait(database))
            == Some(owner)
    }

    fn is_transparent(&self, method: Function) -> bool {
        ROUND_TRIP_TRANSPARENT_METHODS.contains(&method.name(self.semantics.db).as_str())
            && self.is_standard(method)
    }

    fn method_called(
        &self,
        call: &ast::MethodCallExpr,
        body: &SyntaxNode,
    ) -> Option<Trace<'database>> {
        self.method_calls.resolve(call).and_then(
            |SubstitutedResolution {
                 resolution: method, ..
             }| {
                call.receiver()
                    .and_then(|receiver| self.trace(receiver, body))
                    .and_then(|receiver| {
                        if self.is_transparent(method) {
                            Some(receiver)
                        } else if self.is_trait_method(method, self.standard_items.to_string) {
                            receiver.encoded()
                        } else if self.is_text_decoding(method) {
                            receiver.decoded()
                        } else {
                            None
                        }
                    })
            },
        )
    }

    fn operands(&self, root: &SyntaxNode) -> Option<ast::TupleExpr> {
        root.descendants().find_map(|node| {
            ast::MatchExpr::cast(node.clone())
                .and_then(|comparison| comparison.expr())
                .and_then(|scrutinee| ast::TupleExpr::cast(scrutinee.syntax().clone()))
                .or_else(|| {
                    ast::MacroCall::cast(node)
                        .and_then(|call| self.semantics.to_def(&call))
                        .and_then(|expansion| {
                            self.operands(
                                &self.semantics.parse_or_expand(HirFileId::from(expansion)),
                            )
                        })
                })
        })
    }

    fn returns_a_deserialized_value(&self, function: Function) -> bool {
        let returned = function.ret_type(self.semantics.db);

        iter::once(returned.clone())
            .chain(returned.type_arguments())
            .any(|candidate| self.is_bounded_by(&candidate, &self.serialization_traits.deserialize))
    }

    fn trace(&self, expression: ast::Expr, body: &SyntaxNode) -> Option<Trace<'database>> {
        match expression {
            ast::Expr::CallExpr(call) => self.called(&call, body),
            ast::Expr::MethodCallExpr(call) => self.method_called(&call, body),
            ast::Expr::ParenExpr(inner) => inner
                .expr()
                .and_then(|expression| self.trace(expression, body)),
            ast::Expr::PathExpr(path) => {
                path.path()
                    .and_then(|written| match self.paths.resolve(&written) {
                        Some(PathResolution::Local(local)) => Some(self.bound(local, body)),
                        _ => None,
                    })
            }
            ast::Expr::RefExpr(reference) => reference
                .expr()
                .and_then(|expression| self.trace(expression, body)),
            ast::Expr::TryExpr(attempt) => attempt
                .expr()
                .and_then(|expression| self.trace(expression, body)),
            _ => None,
        }
    }
}

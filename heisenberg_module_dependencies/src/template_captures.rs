use std::cell::RefCell;
use std::collections::HashMap;

use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::AstToken;
use ra_ap_syntax::SyntaxToken;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasName;

fn template_call(candidate: &SyntaxToken) -> Option<ast::Expr> {
    candidate
        .parent()
        .and_then(ast::Literal::cast)
        .and_then(|literal| literal.syntax().parent())
        .and_then(ast::Expr::cast)
        .filter(|call| matches!(call, ast::Expr::FormatArgsExpr(_) | ast::Expr::AsmExpr(_)))
}

pub struct TemplateCaptures<'captures, 'database> {
    resolved: RefCell<HashMap<SyntaxToken, Vec<Option<Definition<'database>>>>>,
    semantics: &'captures Semantics<'database, RootDatabase>,
}

impl<'captures, 'database> TemplateCaptures<'captures, 'database> {
    pub fn new(semantics: &'captures Semantics<'database, RootDatabase>) -> Self {
        Self {
            resolved: RefCell::default(),
            semantics,
        }
    }

    pub fn captures(&self, token: &SyntaxToken) -> Vec<Option<Definition<'database>>> {
        let known = self.resolved.borrow().get(token).cloned();

        known.unwrap_or_else(|| {
            let captures = self.resolved_captures(token);

            self.resolved
                .borrow_mut()
                .insert(token.clone(), captures.clone());

            captures
        })
    }

    fn resolved_captures(&self, token: &SyntaxToken) -> Vec<Option<Definition<'database>>> {
        self.semantics
            .descend_into_macros_exact(token.clone())
            .into_iter()
            .filter_map(|candidate| {
                template_call(&candidate).and_then(|call| {
                    let has_named_arguments = ast::FormatArgsExpr::cast(call.syntax().clone())
                        .is_some_and(|format| {
                            format.args().any(|argument| argument.name().is_some())
                        });

                    ast::String::cast(candidate)
                        .and_then(|string| self.semantics.as_format_args_parts(&string))
                        .map(|parts| {
                            parts
                                .into_iter()
                                .filter(|(_, resolution)| {
                                    resolution.is_some() || !has_named_arguments
                                })
                                .map(|(_, resolution)| {
                                    resolution.map(|resolution| {
                                        resolution.either(Definition::from, Definition::from)
                                    })
                                })
                                .collect::<Vec<_>>()
                        })
                })
            })
            .flatten()
            .collect()
    }
}

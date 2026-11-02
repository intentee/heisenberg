use std::convert::Infallible;
use std::iter::once;
use std::ops::ControlFlow;

use ra_ap_cfg::CfgOptions;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_intern::Symbol;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxElement;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::SyntaxToken;
use ra_ap_syntax::T;
use ra_ap_syntax::ast;

use heisenberg_project_model::constants::CLIPPY_DENIED_LINTS;
use heisenberg_project_model::constants::CLIPPY_DENIED_LINTS_GROUP;
use heisenberg_project_model::constants::CLIPPY_TOOL;

use crate::visit_active_metas::visit_active_metas;

const LOWERING_ATTRIBUTES: [&str; 1] = ["warn"];

const SUPPRESSING_ATTRIBUTES: [&str; 2] = ["allow", "expect"];

fn lint_paths(tree: &ast::TokenTree) -> Vec<Vec<String>> {
    let tokens: Vec<SyntaxToken> = tree
        .syntax()
        .children_with_tokens()
        .filter_map(SyntaxElement::into_token)
        .collect();

    tokens
        .split(|token| token.kind() == T![,])
        .map(|lint| {
            lint.iter()
                .filter(|token| token.kind().is_any_identifier())
                .map(|segment| segment.text().to_owned())
                .collect()
        })
        .collect()
}

fn is_required_lint(path: &[String]) -> bool {
    CLIPPY_DENIED_LINTS
        .iter()
        .flat_map(|lint| once(lint.name).chain(lint.former_names.iter().copied()))
        .chain(once(CLIPPY_DENIED_LINTS_GROUP))
        .any(|name| path == [CLIPPY_TOOL, name])
}

pub struct LintSuppressions<'suppressions, 'database> {
    pub node: &'suppressions SyntaxNode,
    pub options: &'suppressions CfgOptions,
    pub semantics: &'suppressions Semantics<'database, RootDatabase>,
}

impl LintSuppressions<'_, '_> {
    pub fn attributes(&self) -> Vec<ast::Meta> {
        let mut suppressions = Vec::new();

        visit_active_metas::<Infallible>(self.node, self.options, |meta| {
            if self.names_builtin_attribute(&meta, &SUPPRESSING_ATTRIBUTES)
                || self.lowers_a_required_lint(&meta)
            {
                suppressions.push(meta);
            }

            ControlFlow::Continue(())
        });

        suppressions
    }

    fn lowers_a_required_lint(&self, meta: &ast::Meta) -> bool {
        self.names_builtin_attribute(meta, &LOWERING_ATTRIBUTES)
            && meta
                .syntax()
                .children()
                .find_map(ast::TokenTree::cast)
                .is_some_and(|tree| lint_paths(&tree).iter().any(|path| is_required_lint(path)))
    }

    fn names_builtin_attribute(&self, meta: &ast::Meta, names: &[&str]) -> bool {
        meta.syntax()
            .children()
            .find_map(ast::Path::cast)
            .is_some_and(|path| {
                matches!(
                    self.semantics.resolve_path(&path),
                    Some(PathResolution::BuiltinAttr(attribute))
                        if names
                            .iter()
                            .any(|name| attribute.name().symbol() == &Symbol::intern(name))
                )
            })
    }
}

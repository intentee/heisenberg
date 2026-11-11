use std::convert::Infallible;
use std::ops::ControlFlow;

use ra_ap_cfg::CfgOptions;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::attrs::Attr;
use ra_ap_hir_expand::attrs::AttrInput;
use ra_ap_hir_expand::mod_path::ModPath;
use ra_ap_hir_expand::mod_path::PathKind;
use ra_ap_ide_db::RootDatabase;
use ra_ap_intern::Interned;
use ra_ap_intern::Symbol;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;
use ra_ap_syntax_bridge::DocCommentDesugarMode;
use ra_ap_syntax_bridge::syntax_node_to_token_tree;

use heisenberg_configured_code::visit_active_metas::visit_active_metas;
use heisenberg_project_model::constants::PANICKING_LINTS;
use heisenberg_project_model::constants::REQUIRED_LINT_GROUPS;

use crate::path_resolutions::PathResolutions;

const LOWERING_ATTRIBUTES: [&str; 1] = ["warn"];

const SUPPRESSING_ATTRIBUTES: [&str; 2] = ["allow", "expect"];

fn is_required_lint(path: &[String]) -> bool {
    REQUIRED_LINT_GROUPS
        .iter()
        .chain(PANICKING_LINTS.iter())
        .any(|lint| lint.is_named_by(path))
}

pub struct LintSuppressions<'suppressions, 'database> {
    pub node: &'suppressions SyntaxNode,
    pub options: &'suppressions CfgOptions,
    pub paths: &'suppressions PathResolutions<'suppressions, 'database>,
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

    fn lint_paths(&self, lowering: &ast::TokenTreeMeta) -> Vec<Vec<String>> {
        let database = self.semantics.db;
        let spans = self
            .semantics
            .hir_file_for(lowering.syntax())
            .span_map(database);
        let attribute = lowering.path().and_then(|path| {
            let span = spans.span_for_range(path.syntax().text_range());

            ModPath::from_tt(
                database,
                syntax_node_to_token_tree(
                    path.syntax(),
                    spans,
                    span,
                    DocCommentDesugarMode::ProcMacro,
                )
                .token_trees(),
            )
            .map(|attribute_path| Attr {
                path: Interned::new(attribute_path),
                input: lowering.token_tree().map(|tree| {
                    Box::new(AttrInput::TokenTree(syntax_node_to_token_tree(
                        tree.syntax(),
                        spans,
                        span,
                        DocCommentDesugarMode::ProcMacro,
                    )))
                }),
                ctxt: span.ctx,
            })
        });

        attribute
            .iter()
            .filter_map(|attribute| attribute.parse_path_comma_token_tree(database))
            .flatten()
            .filter(|(path, ..)| path.kind == PathKind::Plain)
            .map(|(path, ..)| {
                path.segments()
                    .iter()
                    .map(|segment| segment.as_str().to_owned())
                    .collect()
            })
            .collect()
    }

    fn lowers_a_required_lint(&self, meta: &ast::Meta) -> bool {
        let ast::Meta::TokenTreeMeta(lowering) = meta else {
            return false;
        };

        self.names_builtin_attribute(meta, &LOWERING_ATTRIBUTES)
            && self
                .lint_paths(lowering)
                .iter()
                .any(|path| is_required_lint(path))
    }

    fn names_builtin_attribute(&self, meta: &ast::Meta, names: &[&str]) -> bool {
        meta.syntax()
            .children()
            .find_map(ast::Path::cast)
            .is_some_and(|path| {
                matches!(
                    self.paths.resolve(&path),
                    Some(PathResolution::BuiltinAttr(attribute))
                        if names
                            .iter()
                            .any(|name| attribute.name().symbol() == &Symbol::intern(name))
                )
            })
    }
}

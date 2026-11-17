use std::convert::Infallible;
use std::ops::ControlFlow;

use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::attrs::Attr;
use ra_ap_hir_expand::attrs::AttrInput;
use ra_ap_hir_expand::mod_path::ModPath;
use ra_ap_hir_expand::mod_path::PathKind;
use ra_ap_ide_db::RootDatabase;
use ra_ap_intern::Interned;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;
use ra_ap_syntax_bridge::DocCommentDesugarMode;
use ra_ap_syntax_bridge::syntax_node_to_token_tree;

use heisenberg_configured_code::visit_active_metas::visit_active_metas;
use heisenberg_project_model::constants::PANICKING_LINTS;
use heisenberg_project_model::constants::REQUIRED_LINT_GROUPS;

use crate::attribute_directives::AttributeDirectives;
use crate::builtin_attribute::BuiltinAttribute;
use crate::path_resolutions::PathResolutions;
use crate::tool_attribute::ToolAttribute;
use crate::tool_directive::ToolDirective;

const IGNORING_ATTRIBUTES: [&str; 1] = ["ignore"];

const LOWERING_ATTRIBUTES: [&str; 1] = ["warn"];

const SUPPRESSING_ATTRIBUTES: [&str; 2] = ["allow", "expect"];

const VETTED_TOOLS: [&str; 2] = ["clippy", "rustfmt"];

fn is_required_lint(path: &[String]) -> bool {
    REQUIRED_LINT_GROUPS
        .iter()
        .chain(PANICKING_LINTS.iter())
        .any(|lint| lint.is_named_by(path))
}

pub struct NodeAttributes<'attributes, 'database> {
    pub node: &'attributes SyntaxNode,
    pub options: &'attributes CfgOptions,
    pub paths: &'attributes PathResolutions<'attributes, 'database>,
    pub semantics: &'attributes Semantics<'database, RootDatabase>,
}

impl NodeAttributes<'_, '_> {
    pub fn directives(&self) -> AttributeDirectives {
        let mut directives = AttributeDirectives::default();

        visit_active_metas::<Infallible>(self.node, self.options, |meta| {
            let attribute = BuiltinAttribute {
                meta: &meta,
                paths: self.paths,
            };

            if attribute.is_named(&SUPPRESSING_ATTRIBUTES) || self.lowers_a_required_lint(&meta) {
                directives.suppressions.push(meta);
            } else if attribute.is_named(&IGNORING_ATTRIBUTES) {
                directives.ignores.push(meta);
            } else if let Some(tool) = (ToolAttribute {
                meta: &meta,
                paths: self.paths,
                semantics: self.semantics,
            })
            .tool(&VETTED_TOOLS)
            {
                directives
                    .tool_attributes
                    .push(ToolDirective { meta, tool });
            }

            ControlFlow::Continue(())
        });

        directives
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

        BuiltinAttribute {
            meta,
            paths: self.paths,
        }
        .is_named(&LOWERING_ATTRIBUTES)
            && self
                .lint_paths(lowering)
                .iter()
                .any(|path| is_required_lint(path))
    }
}

use std::ops::ControlFlow;

use ra_ap_cfg::CfgOptions;
use ra_ap_hir_expand::attrs::expand_cfg_attr;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

pub fn visit_active_metas<TBreak>(
    node: &SyntaxNode,
    options: &CfgOptions,
    mut visit: impl FnMut(ast::Meta) -> ControlFlow<TBreak>,
) -> Option<TBreak> {
    ast::AnyHasAttrs::cast(node.clone()).and_then(|owner| {
        expand_cfg_attr(
            ast::attrs_including_inner(&owner),
            || options,
            |meta, _| visit(meta),
        )
    })
}

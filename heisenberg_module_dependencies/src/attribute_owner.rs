use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

pub fn attribute_owner(node: &SyntaxNode) -> Option<ast::AnyHasAttrs> {
    ast::AnyHasAttrs::cast(node.clone())
        .or_else(|| ast::Expr::cast(node.clone()).map(ast::AnyHasAttrs::new))
}

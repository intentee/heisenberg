use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

pub struct PathPosition<'position> {
    pub path: &'position ast::Path,
}

impl PathPosition<'_> {
    pub fn construction(&self) -> Option<SyntaxNode> {
        self.path.syntax().parent().filter(|parent| {
            ast::PathExpr::can_cast(parent.kind()) || ast::RecordExpr::can_cast(parent.kind())
        })
    }
}

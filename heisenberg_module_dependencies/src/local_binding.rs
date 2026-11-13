use ra_ap_hir::Local;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

pub fn local_binding<'database>(
    semantics: &Semantics<'database, RootDatabase>,
    root: &SyntaxNode,
    local: Local<'database>,
) -> Option<ast::IdentPat> {
    root.descendants()
        .filter_map(ast::IdentPat::cast)
        .find(|binding| semantics.to_def(binding) == Some(local))
}

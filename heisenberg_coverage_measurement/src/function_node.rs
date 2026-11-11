use ra_ap_hir::HirFileId;
use ra_ap_hir::InFile;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNodePtr;
use ra_ap_syntax::ast;

pub fn function_node(file_id: HirFileId, function: &ast::Fn) -> InFile<SyntaxNodePtr> {
    InFile::new(file_id, SyntaxNodePtr::new(function.syntax()))
}

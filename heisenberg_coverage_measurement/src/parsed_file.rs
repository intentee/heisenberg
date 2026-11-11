use ra_ap_hir::HirFileId;
use ra_ap_syntax::SyntaxNode;

pub struct ParsedFile {
    pub file_id: HirFileId,
    pub root: SyntaxNode,
}

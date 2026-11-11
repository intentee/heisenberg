use ra_ap_ide_db::FilePosition;
use ra_ap_syntax::SyntaxNode;

pub struct EnclosingFunction {
    pub brace: FilePosition,
    pub node: SyntaxNode,
}

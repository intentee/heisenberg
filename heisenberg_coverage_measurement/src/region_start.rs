use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::TextSize;

pub struct RegionStart {
    pub node: SyntaxNode,
    pub offset: TextSize,
}

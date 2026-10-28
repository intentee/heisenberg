use ra_ap_syntax::SyntaxNode;

use crate::subtree_origin::SubtreeOrigin;

pub struct Subtree {
    pub origin: SubtreeOrigin,
    pub root: SyntaxNode,
}

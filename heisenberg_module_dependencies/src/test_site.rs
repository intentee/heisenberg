use ra_ap_hir::Function;
use ra_ap_syntax::SyntaxNode;

use crate::subtree_origin::SubtreeOrigin;

#[derive(Clone)]
pub struct TestSite {
    pub function: Function,
    pub node: SyntaxNode,
    pub origin: SubtreeOrigin,
}

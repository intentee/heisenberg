use std::collections::HashSet;

use ra_ap_hir::Field;
use ra_ap_syntax::SyntaxNode;

#[derive(Default)]
pub struct DerivedSources {
    pub outside_chain: Vec<SyntaxNode>,
    pub sources: HashSet<Field>,
}

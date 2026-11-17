use ra_ap_cfg::CfgAtom;
use ra_ap_syntax::ast;

pub struct NegatedAtom {
    pub atom: CfgAtom,
    pub attribute: ast::Attr,
    pub is_set: bool,
}

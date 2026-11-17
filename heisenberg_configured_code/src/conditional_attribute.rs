use ra_ap_syntax::ast;

pub struct ConditionalAttribute {
    pub applies: bool,
    pub meta: ast::CfgAttrMeta,
}

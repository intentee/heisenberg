use ra_ap_syntax::ast;

#[derive(Default)]
pub struct AttributeDirectives {
    pub expects_a_panic: bool,
    pub ignores: Vec<ast::Meta>,
    pub suppressions: Vec<ast::Meta>,
}

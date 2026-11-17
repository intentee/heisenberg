use ra_ap_syntax::ast;

pub struct ToolDirective {
    pub meta: ast::Meta,
    pub tool: String,
}

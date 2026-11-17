use ra_ap_syntax::ast;

use crate::tool_directive::ToolDirective;

#[derive(Default)]
pub struct AttributeDirectives {
    pub ignores: Vec<ast::Meta>,
    pub suppressions: Vec<ast::Meta>,
    pub tool_attributes: Vec<ToolDirective>,
}

use ra_ap_syntax::ast;

pub enum SubtreeOrigin {
    MacroCall(ast::MacroCall),
    SourceFile,
}

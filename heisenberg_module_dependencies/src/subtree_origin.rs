use ra_ap_syntax::SyntaxNode;

#[derive(Clone)]
pub enum SubtreeOrigin {
    MacroCall(SyntaxNode),
    SourceFile,
}

impl SubtreeOrigin {
    pub fn expansion_of(&self, call: &SyntaxNode) -> Self {
        match self {
            Self::MacroCall(_) => self.clone(),
            Self::SourceFile => Self::MacroCall(call.clone()),
        }
    }
}

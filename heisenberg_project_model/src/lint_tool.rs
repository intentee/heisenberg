#[derive(Clone, Copy, Eq, PartialEq)]
pub enum LintTool {
    Clippy,
    Rust,
}

impl LintTool {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Clippy => "clippy",
            Self::Rust => "rust",
        }
    }
}

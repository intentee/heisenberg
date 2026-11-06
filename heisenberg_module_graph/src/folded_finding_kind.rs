#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FoldedFindingKind {
    FailingExpansion,
    ImplBlock,
    InactiveCode,
    MacroExpansion { depth: usize, name: String },
    SyntaxError { message: String },
    UnreadableLiteral { text: String },
    UnresolvedForeignReferences { count: usize, module: String },
}

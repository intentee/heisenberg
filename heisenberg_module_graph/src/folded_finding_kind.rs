#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FoldedFindingKind {
    FailingExpansion,
    ImplBlock,
    InactiveCode,
    UnresolvedForeignReferences { count: usize, module: String },
}

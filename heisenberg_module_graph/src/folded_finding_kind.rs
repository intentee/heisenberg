#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FoldedFindingKind {
    FailingExpansion,
    ImplBlock,
    InactiveCode,
}

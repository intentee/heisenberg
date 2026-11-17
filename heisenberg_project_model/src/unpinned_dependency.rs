use crate::unpinned_requirement::UnpinnedRequirement;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnpinnedDependency {
    pub dependency: String,
    pub requirement: UnpinnedRequirement,
}

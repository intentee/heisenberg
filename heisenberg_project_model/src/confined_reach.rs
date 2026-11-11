#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ConfinedReach {
    pub manifest: String,
    pub member: String,
    pub package: String,
}

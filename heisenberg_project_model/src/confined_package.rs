use std::collections::BTreeSet;

pub struct ConfinedPackage {
    pub ids: BTreeSet<String>,
    pub users: Vec<String>,
}

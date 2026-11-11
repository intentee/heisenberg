use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::confined_reach::ConfinedReach;

#[derive(Clone, Debug, Default)]
pub struct ConfinedPackageReach {
    pub breaches: BTreeMap<ConfinedReach, Vec<String>>,
    pub users: BTreeSet<ConfinedReach>,
}

use std::collections::BTreeSet;

use crate::violation::Violation;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct EveryConfigurationFindings {
    pub held: BTreeSet<Violation>,
    pub refuted: BTreeSet<Violation>,
}

impl EveryConfigurationFindings {
    pub fn merge(&mut self, Self { held, refuted }: Self) {
        self.held.extend(held);
        self.refuted.extend(refuted);
    }
}

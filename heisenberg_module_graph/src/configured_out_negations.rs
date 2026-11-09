use std::collections::BTreeSet;

use crate::source_location::SourceLocation;
use crate::violation::Violation;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ConfiguredOutNegations {
    pub declarations: BTreeSet<SourceLocation>,
    pub negations: BTreeSet<Violation>,
}

impl ConfiguredOutNegations {
    pub fn merge(
        &mut self,
        Self {
            declarations,
            negations,
        }: Self,
    ) {
        self.declarations.extend(declarations);
        self.negations.extend(negations);
    }
}

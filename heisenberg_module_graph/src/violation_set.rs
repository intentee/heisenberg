use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::btree_map;

use crate::folded_finding::FoldedFinding;
use crate::violation::Violation;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct ViolationSet {
    folded_findings: BTreeMap<Violation, BTreeSet<FoldedFinding>>,
}

impl ViolationSet {
    pub fn fold(&mut self, violation: Violation, finding: FoldedFinding) {
        self.folded_findings
            .entry(violation)
            .or_default()
            .insert(finding);
    }

    pub fn insert(&mut self, violation: Violation) {
        self.folded_findings.entry(violation).or_default();
    }

    pub fn iter(&self) -> btree_map::Iter<'_, Violation, BTreeSet<FoldedFinding>> {
        self.folded_findings.iter()
    }
}

impl Extend<Violation> for ViolationSet {
    fn extend<TViolations: IntoIterator<Item = Violation>>(&mut self, violations: TViolations) {
        for violation in violations {
            self.insert(violation);
        }
    }
}

impl<'set> IntoIterator for &'set ViolationSet {
    type IntoIter = btree_map::Iter<'set, Violation, BTreeSet<FoldedFinding>>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

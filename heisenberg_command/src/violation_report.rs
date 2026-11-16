use heisenberg_module_graph::violation::Violation;

use crate::issue_type::IssueType;
use crate::violation_occurrences::ViolationOccurrences;
use crate::violation_variant::ViolationVariant;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ViolationReport {
    pub occurrences: ViolationOccurrences,
    pub variants: Vec<ViolationVariant>,
    pub violation: Violation,
}

impl ViolationReport {
    #[must_use]
    pub fn new(violation: &Violation, occurrences: ViolationOccurrences) -> Self {
        Self {
            occurrences,
            variants: Vec::new(),
            violation: violation.clone(),
        }
    }

    pub fn absorb(
        &mut self,
        Violation { kind, .. }: &Violation,
        ViolationOccurrences { folded, reported }: ViolationOccurrences,
    ) {
        self.occurrences.folded.extend(folded);
        self.variants.push(ViolationVariant {
            kind: kind.clone(),
            reported,
        });
    }

    #[must_use]
    pub fn reports_the_defect_of(&self, Violation { kind, location }: &Violation) -> bool {
        self.violation.location == *location
            && IssueType::from(&self.violation.kind) == IssueType::from(kind)
    }
}

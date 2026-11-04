use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter;

use heisenberg_module_cycles::non_empty::NonEmpty;

use crate::analysis_context::AnalysisContext;
use crate::issue::Issue;

pub struct IssueSection<'section> {
    pub issue_type: &'section str,
    pub issues: &'section NonEmpty<&'section Issue>,
}

impl IssueSection<'_> {
    #[must_use]
    pub fn lines(
        &self,
        contexts_by_file: &BTreeMap<String, BTreeSet<AnalysisContext>>,
    ) -> Vec<String> {
        let Self { issue_type, issues } = self;
        let count = issues.iter().count();
        let noun = if count == 1 {
            "violation"
        } else {
            "violations"
        };

        iter::once(format!("{issue_type}: {count} {noun}"))
            .chain([
                issues.first.issue_type().text().purpose.to_owned(),
                String::new(),
            ])
            .chain(
                issues
                    .iter()
                    .flat_map(|issue| issue.lines(contexts_by_file)),
            )
            .collect()
    }
}

use std::collections::BTreeMap;

use heisenberg_module_cycles::non_empty::NonEmpty;

use crate::analysis_contexts::AnalysisContexts;
use crate::heisenberg_exit_status::HeisenbergExitStatus;
use crate::issue::Issue;
use crate::issue_section::IssueSection;
use crate::located_issue::LocatedIssue;
use crate::rendered_report::RenderedReport;

const SEPARATOR: &str = "------------------------------";

fn sorted_issues(issues: Vec<Issue>) -> Vec<Issue> {
    let mut located: Vec<LocatedIssue> = issues
        .into_iter()
        .map(|issue| LocatedIssue {
            location: issue.location(),
            issue,
        })
        .collect();

    located.sort();
    located.dedup();

    located
        .into_iter()
        .map(|LocatedIssue { issue, .. }| issue)
        .collect()
}

pub struct IssueReport<'report> {
    pub contexts: &'report AnalysisContexts,
    pub footer: Vec<String>,
    pub issues: Vec<Issue>,
}

impl IssueReport<'_> {
    #[must_use]
    pub fn render(self) -> RenderedReport {
        let Self {
            contexts,
            footer,
            issues,
        } = self;
        let issues = sorted_issues(issues);
        let mut lines: Vec<String> = Vec::new();
        let mut sections: BTreeMap<&str, NonEmpty<&Issue>> = BTreeMap::new();

        for issue in &issues {
            NonEmpty::push_into(&mut sections, issue.issue_type().text().name, issue);
        }

        for issues in sections.values() {
            lines.extend(
                IssueSection {
                    issue_type: issues.first.issue_type(),
                    issues,
                }
                .lines(contexts),
            );
            lines.push(String::new());
        }

        lines.push(SEPARATOR.to_owned());
        lines.push(if issues.is_empty() {
            "No errors found!".to_owned()
        } else {
            format!(
                "{} errors found in {} {}",
                issues.len(),
                sections.len(),
                if sections.len() == 1 { "rule" } else { "rules" }
            )
        });
        lines.push(SEPARATOR.to_owned());
        lines.extend(footer);

        RenderedReport {
            status: if issues.is_empty() {
                HeisenbergExitStatus::Compliant
            } else {
                HeisenbergExitStatus::Violations
            },
            text: lines.join("\n") + "\n",
        }
    }
}

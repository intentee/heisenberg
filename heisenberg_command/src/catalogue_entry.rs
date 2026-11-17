use serde::Serialize;

use crate::issue_text::IssueText;
use crate::issue_type::IssueType;

#[derive(Serialize)]
pub struct CatalogueEntry {
    pub definition: &'static str,
    pub evaluation: String,
    pub exceptions: &'static [&'static str],
    pub identifier: String,
    pub name: &'static str,
    pub rationale: &'static str,
    pub remedy: &'static str,
    pub scope: String,
    pub section: &'static str,
    pub section_number: u8,
    pub see_also: Vec<&'static str>,
    pub summary: &'static str,
}

impl From<IssueType> for CatalogueEntry {
    fn from(issue_type: IssueType) -> Self {
        let text = issue_type.text();
        let IssueText {
            definition,
            evaluation,
            exceptions,
            name,
            rationale,
            remedy,
            scope,
            section,
            summary,
            ..
        } = text;

        Self {
            definition,
            evaluation: evaluation.to_string(),
            exceptions,
            identifier: text.identifier(),
            name,
            rationale,
            remedy,
            scope: scope.to_string(),
            section: section.title(),
            section_number: section.number(),
            see_also: issue_type
                .related()
                .into_iter()
                .map(|related| related.text().name)
                .collect(),
            summary,
        }
    }
}

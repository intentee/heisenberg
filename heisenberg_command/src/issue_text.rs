use crate::rule_evaluation::RuleEvaluation;
use crate::rule_scope::RuleScope;
use crate::rule_section::RuleSection;

const IDENTIFIER_PREFIX: &str = "H";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IssueText {
    pub definition: &'static str,
    pub evaluation: RuleEvaluation,
    pub exceptions: &'static [&'static str],
    pub name: &'static str,
    pub number: u16,
    pub rationale: &'static str,
    pub remedy: &'static str,
    pub scope: RuleScope,
    pub section: RuleSection,
    pub summary: &'static str,
}

impl IssueText {
    #[must_use]
    pub fn identifier(&self) -> String {
        format!("{IDENTIFIER_PREFIX}{:03}", self.number)
    }
}

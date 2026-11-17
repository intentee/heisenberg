use crate::constants::HEISENBERG_VERSION;
use crate::issue_text::IssueText;
use crate::issue_type::IssueType;
use crate::rule_evaluation::RuleEvaluation;

fn names(rules: &[IssueType]) -> String {
    rules
        .iter()
        .map(|issue_type| issue_type.text().name)
        .collect::<Vec<&str>>()
        .join(", ")
}

pub struct ComplianceSummary {
    pub evaluated: Vec<IssueType>,
    pub evaluated_in_part: Vec<IssueType>,
    pub not_evaluated: Vec<IssueType>,
}

impl ComplianceSummary {
    #[must_use]
    pub fn new_full(rules: &[IssueType]) -> Self {
        Self {
            evaluated: rules.to_vec(),
            evaluated_in_part: Vec::new(),
            not_evaluated: Vec::new(),
        }
    }

    #[must_use]
    pub fn new_partial(rules: &[IssueType], decided: &[IssueType]) -> Self {
        let mut summary = Self::new_full(&[]);

        for issue_type in rules {
            let IssueText { evaluation, .. } = issue_type.text();

            if decided.contains(issue_type) {
                summary.evaluated.push(*issue_type);
            } else {
                match evaluation {
                    RuleEvaluation::EachAnalysedConfiguration => &mut summary.evaluated,
                    RuleEvaluation::EveryDeclaredConfiguration => &mut summary.not_evaluated,
                    RuleEvaluation::EveryDeclaredConfigurationInPart => {
                        &mut summary.evaluated_in_part
                    }
                }
                .push(*issue_type);
            }
        }

        summary
    }

    #[must_use]
    pub fn partial_run_lines(&self) -> Vec<String> {
        let Self {
            evaluated_in_part,
            not_evaluated,
            ..
        } = self;

        vec![
            format!("Not evaluated in a partial run: {}", names(not_evaluated)),
            format!(
                "Evaluated in part in a partial run: {}",
                names(evaluated_in_part)
            ),
        ]
    }

    #[must_use]
    pub fn summary_lines(&self) -> Vec<String> {
        let Self {
            evaluated,
            evaluated_in_part,
            not_evaluated,
        } = self;

        vec![
            format!(
                "Rules: {} evaluated, {} evaluated in part, {} not evaluated",
                evaluated.len(),
                evaluated_in_part.len(),
                not_evaluated.len()
            ),
            format!("Heisenberg {HEISENBERG_VERSION}"),
        ]
    }
}

use std::collections::BTreeMap;
use std::io::Write;
use std::iter;

use clap::ValueEnum;
use serde_json::Error as SerdeJsonError;
use serde_json::to_writer_pretty;

use crate::catalogue_document::CatalogueDocument;
use crate::catalogue_entry::CatalogueEntry;
use crate::constants::HEISENBERG_VERSION;
use crate::constants::RULE_CATEGORY;
use crate::constants::RULE_DECIDABILITY;
use crate::issue_text::IssueText;
use crate::issue_type::IssueType;
use crate::rule_section::RuleSection;

pub struct RuleCatalogue {
    pub sections: BTreeMap<RuleSection, BTreeMap<u16, IssueType>>,
}

impl RuleCatalogue {
    #[must_use]
    pub fn new(includes: fn(&IssueType) -> bool) -> Self {
        let mut sections: BTreeMap<RuleSection, BTreeMap<u16, IssueType>> = BTreeMap::new();

        for issue_type in IssueType::value_variants()
            .iter()
            .filter(|issue_type| includes(issue_type))
        {
            let IssueText {
                number, section, ..
            } = issue_type.text();

            sections
                .entry(section)
                .or_default()
                .insert(number, *issue_type);
        }

        Self { sections }
    }

    #[must_use]
    pub fn listing(&self) -> String {
        self.sections
            .iter()
            .flat_map(|(section, rules)| {
                iter::once(format!("  {} {}", section.number(), section.title())).chain(
                    rules.values().map(|issue_type| {
                        let text = issue_type.text();

                        format!("    {} {}: {}", text.identifier(), text.name, text.summary)
                    }),
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[must_use]
    pub fn text(&self) -> String {
        [
            format!("Heisenberg {HEISENBERG_VERSION} rule catalogue"),
            format!(
                "Every rule is {RULE_CATEGORY} and {RULE_DECIDABILITY}: no configuration relaxes or exempts from a rule, and every rule is decided from the declared inputs alone."
            ),
        ]
        .into_iter()
        .chain(self.sections.iter().flat_map(|(section, rules)| {
            [
                String::new(),
                format!("Section {}: {}", section.number(), section.title()),
            ]
            .into_iter()
            .chain(rules.values().flat_map(|issue_type| {
                let text = issue_type.text();

                [
                    String::new(),
                    format!("{} {}: {}", text.identifier(), text.name, text.summary),
                    format!("  Scope: {}", text.scope),
                    format!("  Applies to: {}", issue_type.applicability()),
                    format!("  Evaluated: {}", text.evaluation),
                ]
            }))
        }))
        .collect::<Vec<String>>()
        .join("\n")
            + "\n"
    }

    /// # Errors
    ///
    /// Returns the [`SerdeJsonError`] of writing the document to `output`.
    pub fn write_json(&self, output: &mut dyn Write) -> Result<(), SerdeJsonError> {
        to_writer_pretty(
            output,
            &CatalogueDocument {
                category: RULE_CATEGORY,
                decidability: RULE_DECIDABILITY,
                rules: self
                    .sections
                    .values()
                    .flat_map(BTreeMap::values)
                    .copied()
                    .map(CatalogueEntry::from)
                    .collect(),
                version: HEISENBERG_VERSION,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    use clap::ValueEnum;

    use super::RuleCatalogue;
    use crate::issue_type::IssueType;

    #[test]
    fn every_rule_has_its_own_identifier() {
        assert_eq!(
            RuleCatalogue::new(|_| true)
                .sections
                .values()
                .flat_map(BTreeMap::keys)
                .collect::<BTreeSet<&u16>>()
                .len(),
            IssueType::value_variants().len()
        );
    }
}

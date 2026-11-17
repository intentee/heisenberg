use heisenberg_module_graph::layout_violation::LayoutViolation;

use crate::constants::IMPORT_INSIDE_BLOCK;
use crate::constants::POSITIONAL_FIELDS;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutIssue {
    ImportInsideBlock,
    PositionalFields,
}

impl LayoutIssue {
    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::ImportInsideBlock => IMPORT_INSIDE_BLOCK,
            Self::PositionalFields => POSITIONAL_FIELDS,
        }
    }

    #[must_use]
    pub const fn variants() -> [Self; 2] {
        [Self::ImportInsideBlock, Self::PositionalFields]
    }
}

impl From<&LayoutViolation> for LayoutIssue {
    fn from(violation: &LayoutViolation) -> Self {
        match violation {
            LayoutViolation::ImportInsideBlock => Self::ImportInsideBlock,
            LayoutViolation::PositionalFields => Self::PositionalFields,
        }
    }
}

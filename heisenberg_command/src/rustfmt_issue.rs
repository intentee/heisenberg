use heisenberg_project_model::rustfmt_violation::RustfmtViolation;

use crate::constants::RUSTFMT_CONFIGURATION_AMBIGUOUS;
use crate::constants::RUSTFMT_CONFIGURATION_INVALID;
use crate::constants::RUSTFMT_CONFIGURATION_MISSING;
use crate::constants::RUSTFMT_CONFIGURATION_UNREADABLE;
use crate::constants::UNVETTED_RUSTFMT_SETTING;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RustfmtIssue {
    ConfigurationAmbiguous,
    ConfigurationInvalid,
    ConfigurationMissing,
    ConfigurationUnreadable,
    UnvettedSetting,
}

impl RustfmtIssue {
    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::ConfigurationAmbiguous => RUSTFMT_CONFIGURATION_AMBIGUOUS,
            Self::ConfigurationInvalid => RUSTFMT_CONFIGURATION_INVALID,
            Self::ConfigurationMissing => RUSTFMT_CONFIGURATION_MISSING,
            Self::ConfigurationUnreadable => RUSTFMT_CONFIGURATION_UNREADABLE,
            Self::UnvettedSetting => UNVETTED_RUSTFMT_SETTING,
        }
    }

    #[must_use]
    pub const fn variants() -> [Self; 5] {
        [
            Self::ConfigurationAmbiguous,
            Self::ConfigurationInvalid,
            Self::ConfigurationMissing,
            Self::ConfigurationUnreadable,
            Self::UnvettedSetting,
        ]
    }
}

impl From<&RustfmtViolation> for RustfmtIssue {
    fn from(violation: &RustfmtViolation) -> Self {
        match violation {
            RustfmtViolation::ConfigurationAmbiguous { .. } => Self::ConfigurationAmbiguous,
            RustfmtViolation::ConfigurationInvalid { .. } => Self::ConfigurationInvalid,
            RustfmtViolation::ConfigurationMissing { .. } => Self::ConfigurationMissing,
            RustfmtViolation::ConfigurationUnreadable { .. } => Self::ConfigurationUnreadable,
            RustfmtViolation::UnvettedSetting { .. } => Self::UnvettedSetting,
        }
    }
}

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::NEXTEST_CONFIGURATION_DIRECTORY;
use crate::constants::NEXTEST_CONFIGURATION_FILE;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NextestPresenceViolation {
    Missing { manifest: String },
    Unneeded { file: String },
}

impl NextestPresenceViolation {
    #[must_use]
    pub fn file(&self) -> &str {
        match self {
            Self::Missing { manifest: file } | Self::Unneeded { file } => file,
        }
    }
}

impl Display for NextestPresenceViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing { .. } => write!(
                formatter,
                "Workspace has no {NEXTEST_CONFIGURATION_DIRECTORY}/{NEXTEST_CONFIGURATION_FILE}"
            ),
            Self::Unneeded { .. } => formatter.write_str(
                "Nextest configuration of a workspace that runs no test under libtest's harness",
            ),
        }
    }
}

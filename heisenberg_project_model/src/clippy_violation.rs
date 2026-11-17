use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::constants::CLIPPY_CONFIGURATION_FILE;
use crate::constants::CLIPPY_HIDDEN_CONFIGURATION_FILE;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ClippyViolation {
    ConfigurationAmbiguous {
        file: String,
    },
    ConfigurationMissing {
        manifest: String,
        package: String,
    },
    RequiredSettingMissing {
        file: String,
        setting: String,
        value: bool,
    },
    UnvettedSetting {
        file: String,
        setting: String,
    },
}

impl ClippyViolation {
    #[must_use]
    pub fn file(&self) -> &str {
        match self {
            Self::ConfigurationAmbiguous { file }
            | Self::ConfigurationMissing { manifest: file, .. }
            | Self::RequiredSettingMissing { file, .. }
            | Self::UnvettedSetting { file, .. } => file,
        }
    }
}

impl Display for ClippyViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigurationAmbiguous { .. } => write!(
                formatter,
                "Directory holds both {CLIPPY_CONFIGURATION_FILE} and {CLIPPY_HIDDEN_CONFIGURATION_FILE}"
            ),
            Self::ConfigurationMissing { package, .. } => write!(
                formatter,
                "Package {package} has no {CLIPPY_CONFIGURATION_FILE} or {CLIPPY_HIDDEN_CONFIGURATION_FILE} within the workspace"
            ),
            Self::RequiredSettingMissing { setting, value, .. } => {
                write!(
                    formatter,
                    "Clippy configuration does not set {setting} = {value}"
                )
            }
            Self::UnvettedSetting { setting, .. } => write!(
                formatter,
                "Clippy configuration sets {setting}, which Heisenberg has not vetted"
            ),
        }
    }
}

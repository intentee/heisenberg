use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::ErrorKind;

use crate::constants::RUSTFMT_CONFIGURATION_FILE;
use crate::constants::RUSTFMT_HIDDEN_CONFIGURATION_FILE;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RustfmtViolation {
    ConfigurationAmbiguous { file: String },
    ConfigurationInvalid { file: String, reason: String },
    ConfigurationMissing { manifest: String },
    ConfigurationUnreadable { file: String, kind: ErrorKind },
    UnvettedSetting { file: String, setting: String },
}

impl RustfmtViolation {
    #[must_use]
    pub fn file(&self) -> &str {
        match self {
            Self::ConfigurationAmbiguous { file }
            | Self::ConfigurationInvalid { file, .. }
            | Self::ConfigurationMissing { manifest: file }
            | Self::ConfigurationUnreadable { file, .. }
            | Self::UnvettedSetting { file, .. } => file,
        }
    }
}

impl Display for RustfmtViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigurationAmbiguous { .. } => write!(
                formatter,
                "Directory holds both {RUSTFMT_CONFIGURATION_FILE} and {RUSTFMT_HIDDEN_CONFIGURATION_FILE}"
            ),
            Self::ConfigurationInvalid { reason, .. } => {
                write!(
                    formatter,
                    "Rustfmt configuration is not valid TOML: {reason}"
                )
            }
            Self::ConfigurationMissing { .. } => write!(
                formatter,
                "Workspace root holds neither {RUSTFMT_CONFIGURATION_FILE} nor {RUSTFMT_HIDDEN_CONFIGURATION_FILE}"
            ),
            Self::ConfigurationUnreadable { kind, .. } => {
                write!(formatter, "Rustfmt configuration cannot be read: {kind}")
            }
            Self::UnvettedSetting { setting, .. } => write!(
                formatter,
                "Setting {setting} is not one that Heisenberg has vetted"
            ),
        }
    }
}

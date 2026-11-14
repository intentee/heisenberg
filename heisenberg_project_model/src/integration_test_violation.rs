use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IntegrationTestViolation {
    Discovered {
        manifest: String,
    },
    Split {
        manifest: String,
        targets: Vec<String>,
    },
}

impl IntegrationTestViolation {
    #[must_use]
    pub fn manifest(&self) -> &str {
        match self {
            Self::Discovered { manifest } | Self::Split { manifest, .. } => manifest,
        }
    }
}

impl Display for IntegrationTestViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Discovered { .. } => formatter.write_str(
                "Package builds integration test targets with libtest's harness without setting autotests = false",
            ),
            Self::Split { targets, .. } => write!(
                formatter,
                "Package builds {} integration test targets with libtest's harness instead of one",
                targets.len()
            ),
        }
    }
}

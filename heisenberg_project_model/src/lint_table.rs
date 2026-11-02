use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LintTable {
    Package { manifest: String, name: String },
    Workspace { manifest: String },
}

impl LintTable {
    #[must_use]
    pub fn manifest(&self) -> &str {
        match self {
            Self::Package { manifest, .. } | Self::Workspace { manifest } => manifest,
        }
    }
}

impl Display for LintTable {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Package { name, .. } => write!(formatter, "Package {name}"),
            Self::Workspace { .. } => formatter.write_str("Workspace"),
        }
    }
}

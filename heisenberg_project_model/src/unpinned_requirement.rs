use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UnpinnedRequirement {
    GitWithoutRevision,
    Version { requirement: String },
}

impl Display for UnpinnedRequirement {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::GitWithoutRevision => {
                formatter.write_str("follows a git repository without naming a rev")
            }
            Self::Version { requirement } => write!(
                formatter,
                "requires `{requirement}`, which is not a MAJOR.MINOR version, or the MAJOR.MINOR.PATCH of a 0.0 or pre-release version"
            ),
        }
    }
}

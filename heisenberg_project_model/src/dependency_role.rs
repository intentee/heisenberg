use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DependencyRole {
    Build,
    Development,
    Normal,
}

impl Display for DependencyRole {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Build => "Build-dependency",
            Self::Development => "Dev-dependency",
            Self::Normal => "Dependency",
        })
    }
}

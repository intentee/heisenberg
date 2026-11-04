use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PotentialOwner {
    Blocked { cycle: Vec<String>, package: String },
    Free { package: String },
}

impl Display for PotentialOwner {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blocked { cycle, package } => {
                write!(
                    formatter,
                    "cannot move to {package}: {}",
                    cycle.join(" -> ")
                )
            }
            Self::Free { package } => write!(
                formatter,
                "could move without a dependency cycle to {package}"
            ),
        }
    }
}

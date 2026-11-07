use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::method_group::MethodGroup;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MethodPlacement {
    Declared {
        preceding: String,
        trait_name: String,
    },
    Grouped {
        group: MethodGroup,
        preceding: String,
        preceding_group: MethodGroup,
    },
}

impl Display for MethodPlacement {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Declared {
                preceding,
                trait_name,
            } => write!(
                formatter,
                "Method belongs before {preceding}: {trait_name} declares it earlier"
            ),
            Self::Grouped {
                group,
                preceding,
                preceding_group,
            } if group == preceding_group => write!(
                formatter,
                "Method belongs before {preceding}: {group} are sorted by name"
            ),
            Self::Grouped {
                group,
                preceding,
                preceding_group,
            } => write!(
                formatter,
                "Method belongs before {preceding}: {group} precede {preceding_group}"
            ),
        }
    }
}

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::method_group::MethodGroup;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MethodPlacement {
    pub group: MethodGroup,
    pub preceding: String,
    pub preceding_group: MethodGroup,
}

impl Display for MethodPlacement {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let Self {
            group,
            preceding,
            preceding_group,
        } = self;

        if group == preceding_group {
            write!(
                formatter,
                "Method belongs before {preceding}: {group} are sorted by name"
            )
        } else {
            write!(
                formatter,
                "Method belongs before {preceding}: {group} precede {preceding_group}"
            )
        }
    }
}

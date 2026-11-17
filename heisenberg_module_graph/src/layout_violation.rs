use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LayoutViolation {
    ImportInsideBlock,
    PositionalFields,
}

impl Display for LayoutViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ImportInsideBlock => "Import is written inside a block",
            Self::PositionalFields => "Fields are positional",
        })
    }
}

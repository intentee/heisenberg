use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LinePosition {
    pub line: u64,
    pub column: u64,
    pub source_line: String,
}

impl Display for LinePosition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let Self { line, column, .. } = self;

        write!(formatter, "{line}:{column}")
    }
}

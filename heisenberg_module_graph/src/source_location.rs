use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::line_position::LinePosition;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceLocation {
    pub file: String,
    pub position: LinePosition,
}

impl Display for SourceLocation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let Self { file, position } = self;

        write!(formatter, "{file}:{position}")
    }
}

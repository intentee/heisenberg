use crate::source_location::SourceLocation;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InactiveRegion {
    pub start: SourceLocation,
    pub end: SourceLocation,
}

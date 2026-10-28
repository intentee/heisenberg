use crate::source_location::SourceLocation;
use crate::violation_kind::ViolationKind;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Violation {
    pub location: SourceLocation,
    pub kind: ViolationKind,
}

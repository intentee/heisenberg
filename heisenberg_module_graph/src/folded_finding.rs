use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::folded_finding_kind::FoldedFindingKind;
use crate::source_location::SourceLocation;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FoldedFinding {
    pub kind: FoldedFindingKind,
    pub location: SourceLocation,
}

impl Display for FoldedFinding {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            FoldedFindingKind::FailingExpansion => {
                write!(formatter, "expansion failing at {}", self.location)
            }
            FoldedFindingKind::ImplBlock => write!(formatter, "impl block at {}", self.location),
            FoldedFindingKind::InactiveCode => write!(
                formatter,
                "code active on no declared target at {}",
                self.location
            ),
        }
    }
}

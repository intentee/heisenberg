use std::collections::BTreeSet;

use crate::source_location::SourceLocation;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct InactiveFocus {
    pub location: SourceLocation,
    pub token_starts: BTreeSet<u32>,
}

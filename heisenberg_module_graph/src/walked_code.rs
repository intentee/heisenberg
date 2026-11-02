use std::collections::BTreeSet;

use crate::inactive_region::InactiveRegion;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct WalkedCode {
    pub files: BTreeSet<String>,
    pub inactive_regions: BTreeSet<InactiveRegion>,
}

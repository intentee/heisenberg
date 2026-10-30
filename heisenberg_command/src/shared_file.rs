use std::collections::BTreeSet;

use crate::crate_module::CrateModule;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SharedFile {
    pub file: String,
    pub modules: BTreeSet<CrateModule>,
}

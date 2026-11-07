use std::collections::BTreeSet;

use crate::configuration::Configuration;

#[derive(Clone)]
pub struct RunScope {
    pub left_out_configurations: BTreeSet<Configuration>,
}

impl RunScope {
    #[must_use]
    pub fn is_partial(&self) -> bool {
        !self.left_out_configurations.is_empty()
    }
}

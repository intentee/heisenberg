use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use heisenberg_project_model::supported_target::SupportedTarget;

use crate::target_list::TargetList;

pub struct TargetNarrowing<'targets> {
    pub all: &'targets BTreeSet<SupportedTarget>,
    pub targets: &'targets BTreeSet<SupportedTarget>,
}

impl Display for TargetNarrowing<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        if self.targets == self.all {
            Ok(())
        } else {
            write!(
                formatter,
                " (only on {})",
                TargetList {
                    targets: self.targets
                }
            )
        }
    }
}

use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use heisenberg_project_model::supported_target::SupportedTarget;

pub struct TargetList<'targets> {
    pub targets: &'targets BTreeSet<SupportedTarget>,
}

impl Display for TargetList<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            &self
                .targets
                .iter()
                .map(|target| target.triple())
                .collect::<Vec<&str>>()
                .join(", "),
        )
    }
}

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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use heisenberg_project_model::supported_target::SupportedTarget;

    use super::TargetNarrowing;

    #[test]
    fn targets_narrower_than_every_analysed_target_are_named() {
        assert_eq!(
            TargetNarrowing {
                all: &BTreeSet::from([
                    SupportedTarget::X86_64PcWindowsMsvc,
                    SupportedTarget::X86_64UnknownLinuxGnu,
                ]),
                targets: &BTreeSet::from([SupportedTarget::X86_64PcWindowsMsvc]),
            }
            .to_string(),
            " (only on x86_64-pc-windows-msvc)"
        );
    }
}

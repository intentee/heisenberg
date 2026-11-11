use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::crate_target_kind::CrateTargetKind;
use crate::test_harness::TestHarness;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CrateTarget {
    pub name: String,
    pub package_name: String,
    pub target_description: String,
    pub kind: CrateTargetKind,
    pub test_harness: TestHarness,
}

impl CrateTarget {
    #[must_use]
    pub fn enters_through_main(&self) -> bool {
        match self.kind {
            CrateTargetKind::Binary | CrateTargetKind::BuildScript | CrateTargetKind::Example => {
                true
            }
            CrateTargetKind::Benchmark | CrateTargetKind::IntegrationTest => {
                self.test_harness == TestHarness::Absent
            }
            CrateTargetKind::Library => false,
        }
    }

    #[must_use]
    pub fn is_test_code(&self, test_crates: &BTreeSet<String>) -> bool {
        self.kind == CrateTargetKind::IntegrationTest || test_crates.contains(&self.package_name)
    }
}

impl Display for CrateTarget {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let Self {
            package_name,
            target_description,
            ..
        } = self;

        write!(formatter, "{package_name} {target_description}")
    }
}

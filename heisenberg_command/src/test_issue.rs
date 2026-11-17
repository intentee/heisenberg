use heisenberg_module_graph::test_violation::TestViolation;

use crate::constants::IGNORED_TEST;
use crate::constants::TEST_MODULE_IN_ITS_OWN_FILE;
use crate::constants::TEST_MODULE_NOT_NAMED_TESTS;
use crate::constants::TEST_OUTSIDE_TEST_MODULE;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TestIssue {
    Ignored,
    ModuleInItsOwnFile,
    ModuleNotNamedTests,
    OutsideTestModule,
}

impl TestIssue {
    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::Ignored => IGNORED_TEST,
            Self::ModuleInItsOwnFile => TEST_MODULE_IN_ITS_OWN_FILE,
            Self::ModuleNotNamedTests => TEST_MODULE_NOT_NAMED_TESTS,
            Self::OutsideTestModule => TEST_OUTSIDE_TEST_MODULE,
        }
    }

    #[must_use]
    pub const fn variants() -> [Self; 4] {
        [
            Self::Ignored,
            Self::ModuleInItsOwnFile,
            Self::ModuleNotNamedTests,
            Self::OutsideTestModule,
        ]
    }
}

impl From<&TestViolation> for TestIssue {
    fn from(violation: &TestViolation) -> Self {
        match violation {
            TestViolation::Ignored => Self::Ignored,
            TestViolation::ModuleInItsOwnFile => Self::ModuleInItsOwnFile,
            TestViolation::ModuleNotNamedTests => Self::ModuleNotNamedTests,
            TestViolation::OutsideTestModule => Self::OutsideTestModule,
        }
    }
}

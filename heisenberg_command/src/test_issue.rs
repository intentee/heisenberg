use crate::constants::IGNORED_TEST;
use crate::constants::SERIALIZATION_ROUND_TRIP_TEST;
use crate::constants::SLEEP_IN_TEST;
use crate::constants::TEST_WITHOUT_ASSERTION;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TestIssue {
    IgnoredTest,
    SerializationRoundTripTest,
    SleepInTest,
    TestWithoutAssertion,
}

impl TestIssue {
    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::IgnoredTest => IGNORED_TEST,
            Self::SerializationRoundTripTest => SERIALIZATION_ROUND_TRIP_TEST,
            Self::SleepInTest => SLEEP_IN_TEST,
            Self::TestWithoutAssertion => TEST_WITHOUT_ASSERTION,
        }
    }
}

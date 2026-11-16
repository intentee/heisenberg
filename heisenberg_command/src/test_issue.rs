use crate::constants::IGNORED_TEST;
use crate::constants::SERIALIZATION_ROUND_TRIP_TEST;
use crate::constants::SLEEP_IN_TEST;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TestIssue {
    Ignored,
    SerializationRoundTrip,
    Sleeping,
}

impl TestIssue {
    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::Ignored => IGNORED_TEST,
            Self::SerializationRoundTrip => SERIALIZATION_ROUND_TRIP_TEST,
            Self::Sleeping => SLEEP_IN_TEST,
        }
    }
}

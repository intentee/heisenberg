use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TestViolation {
    Ignored,
    SerializationRoundTrip,
    Sleeping,
}

impl Display for TestViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Ignored => "Test skipped by an ignore attribute",
            Self::SerializationRoundTrip => {
                "Test only compares values with what their serialization round trip decodes"
            }
            Self::Sleeping => "Test code sleeps",
        })
    }
}

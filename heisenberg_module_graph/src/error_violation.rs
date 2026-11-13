use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorViolation {
    Discarded,
    Handwritten,
    MultipleTypes { count: usize },
    SourceInMessage,
    SourceOutsideChain,
    UntypedResult { error_type: String },
}

impl Display for ErrorViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Discarded => formatter.write_str("Error is dropped without being handled"),
            Self::Handwritten => {
                formatter.write_str("Error type is not an enum that derives thiserror::Error")
            }
            Self::MultipleTypes { count } => {
                write!(
                    formatter,
                    "Error type is one of the {count} that the crate implements"
                )
            }
            Self::SourceInMessage => formatter.write_str(
                "Error message formats the source that the error chain reports after it",
            ),
            Self::SourceOutsideChain => {
                formatter.write_str("Field holds an error outside the source chain of its variant")
            }
            Self::UntypedResult { error_type } => write!(
                formatter,
                "Result error type {error_type} does not implement std::error::Error"
            ),
        }
    }
}

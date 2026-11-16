use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::untyped_error_name::UntypedErrorName;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorViolation {
    Handwritten,
    MacroGenerated,
    MultipleTypes { count: usize },
    NotAnEnum,
    SourceInMessage,
    SourceOutsideChain,
    UntypedResult { error: UntypedErrorName },
}

impl Display for ErrorViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Handwritten => formatter.write_str("Error type is implemented by hand"),
            Self::MacroGenerated => formatter
                .write_str("Error type is implemented by a macro other than thiserror::Error"),
            Self::MultipleTypes { count } => {
                write!(
                    formatter,
                    "Error type is one of the {count} that the crate implements"
                )
            }
            Self::NotAnEnum => {
                formatter.write_str("Error type that derives thiserror::Error is not an enum")
            }
            Self::SourceInMessage => formatter.write_str(
                "Error message formats the source that the error chain reports after it",
            ),
            Self::SourceOutsideChain => {
                formatter.write_str("Field holds an error outside the source chain of its variant")
            }
            Self::UntypedResult { error } => write!(
                formatter,
                "Result error type {error} does not implement std::error::Error"
            ),
        }
    }
}

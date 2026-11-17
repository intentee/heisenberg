use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::error_issuance::ErrorIssuance;
use crate::module_path::ModulePath;
use crate::untyped_error_name::UntypedErrorName;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorViolation {
    Handwritten,
    HandwrittenDisplay,
    IssuedOutsideItsCrate {
        issuance: ErrorIssuance,
        owner: String,
    },
    MacroGenerated,
    MessageFunction,
    MessageOnTheEnum,
    MultipleTypes {
        count: usize,
    },
    NotAnEnum,
    NotAnError {
        error: UntypedErrorName,
    },
    OutsideItsModule {
        module: ModulePath,
    },
    SourceInMessage,
    SourceOutsideChain,
    UntypedResult {
        error: UntypedErrorName,
    },
}

impl Display for ErrorViolation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Handwritten => formatter.write_str("Error type is implemented by hand"),
            Self::HandwrittenDisplay => {
                formatter.write_str("Error type formats its message by hand")
            }
            Self::IssuedOutsideItsCrate {
                issuance: ErrorIssuance::Constructed { written },
                owner,
            } => write!(
                formatter,
                "Error {written} belongs to the crate {owner}, and only that crate constructs it"
            ),
            Self::IssuedOutsideItsCrate {
                issuance: ErrorIssuance::Returned { error },
                owner,
            } => write!(
                formatter,
                "Result error type {error} belongs to the crate {owner}, and only that crate returns it"
            ),
            Self::MessageFunction => {
                formatter.write_str("Error message is formatted by a function")
            }
            Self::MessageOnTheEnum => {
                formatter.write_str("Error message is written on the enum instead of its variants")
            }
            Self::OutsideItsModule { module } => write!(
                formatter,
                "Error type is defined in {module} instead of a module file named after it"
            ),
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
            Self::NotAnError { error } => write!(
                formatter,
                "Result error type {error} is a type of this workspace that is not an error"
            ),
        }
    }
}

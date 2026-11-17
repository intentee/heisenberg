use heisenberg_module_graph::error_violation::ErrorViolation;

use crate::constants::ERROR_MESSAGE_FUNCTION;
use crate::constants::ERROR_MESSAGE_ON_THE_ENUM;
use crate::constants::ERROR_SOURCE_OUTSIDE_CHAIN;
use crate::constants::ERROR_TYPE_NOT_AN_ENUM;
use crate::constants::ERROR_TYPE_OUTSIDE_ITS_MODULE;
use crate::constants::HANDWRITTEN_ERROR_DISPLAY;
use crate::constants::HANDWRITTEN_ERROR_TYPE;
use crate::constants::MACRO_GENERATED_ERROR_TYPE;
use crate::constants::MULTIPLE_ERROR_TYPES;
use crate::constants::RESULT_ERROR_NOT_AN_ERROR;
use crate::constants::SOURCE_IN_ERROR_MESSAGE;
use crate::constants::UNTYPED_RESULT_ERROR;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorIssue {
    ErrorMessageFunction,
    ErrorMessageOnTheEnum,
    ErrorSourceOutsideChain,
    ErrorTypeNotAnEnum,
    ErrorTypeOutsideItsModule,
    HandwrittenErrorDisplay,
    HandwrittenErrorType,
    MacroGeneratedErrorType,
    MultipleErrorTypes,
    ResultErrorNotAnError,
    SourceInErrorMessage,
    UntypedResultError,
}

impl ErrorIssue {
    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::ErrorMessageFunction => ERROR_MESSAGE_FUNCTION,
            Self::ErrorMessageOnTheEnum => ERROR_MESSAGE_ON_THE_ENUM,
            Self::ErrorSourceOutsideChain => ERROR_SOURCE_OUTSIDE_CHAIN,
            Self::ErrorTypeNotAnEnum => ERROR_TYPE_NOT_AN_ENUM,
            Self::ErrorTypeOutsideItsModule => ERROR_TYPE_OUTSIDE_ITS_MODULE,
            Self::HandwrittenErrorDisplay => HANDWRITTEN_ERROR_DISPLAY,
            Self::HandwrittenErrorType => HANDWRITTEN_ERROR_TYPE,
            Self::MacroGeneratedErrorType => MACRO_GENERATED_ERROR_TYPE,
            Self::MultipleErrorTypes => MULTIPLE_ERROR_TYPES,
            Self::ResultErrorNotAnError => RESULT_ERROR_NOT_AN_ERROR,
            Self::SourceInErrorMessage => SOURCE_IN_ERROR_MESSAGE,
            Self::UntypedResultError => UNTYPED_RESULT_ERROR,
        }
    }

    #[must_use]
    pub const fn variants() -> [Self; 12] {
        [
            Self::ErrorMessageFunction,
            Self::ErrorMessageOnTheEnum,
            Self::ErrorSourceOutsideChain,
            Self::ErrorTypeNotAnEnum,
            Self::ErrorTypeOutsideItsModule,
            Self::HandwrittenErrorDisplay,
            Self::HandwrittenErrorType,
            Self::MacroGeneratedErrorType,
            Self::MultipleErrorTypes,
            Self::ResultErrorNotAnError,
            Self::SourceInErrorMessage,
            Self::UntypedResultError,
        ]
    }
}

impl From<&ErrorViolation> for ErrorIssue {
    fn from(violation: &ErrorViolation) -> Self {
        match violation {
            ErrorViolation::Handwritten => Self::HandwrittenErrorType,
            ErrorViolation::HandwrittenDisplay => Self::HandwrittenErrorDisplay,
            ErrorViolation::MessageFunction => Self::ErrorMessageFunction,
            ErrorViolation::MessageOnTheEnum => Self::ErrorMessageOnTheEnum,
            ErrorViolation::OutsideItsModule { .. } => Self::ErrorTypeOutsideItsModule,
            ErrorViolation::MacroGenerated => Self::MacroGeneratedErrorType,
            ErrorViolation::MultipleTypes { .. } => Self::MultipleErrorTypes,
            ErrorViolation::NotAnEnum => Self::ErrorTypeNotAnEnum,
            ErrorViolation::SourceInMessage => Self::SourceInErrorMessage,
            ErrorViolation::SourceOutsideChain => Self::ErrorSourceOutsideChain,
            ErrorViolation::NotAnError { .. } => Self::ResultErrorNotAnError,
            ErrorViolation::UntypedResult { .. } => Self::UntypedResultError,
        }
    }
}

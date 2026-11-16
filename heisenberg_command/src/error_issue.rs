use heisenberg_module_graph::error_violation::ErrorViolation;

use crate::constants::DISCARDED_ERROR;
use crate::constants::ERROR_SOURCE_OUTSIDE_CHAIN;
use crate::constants::ERROR_TYPE_NOT_AN_ENUM;
use crate::constants::HANDWRITTEN_ERROR_TYPE;
use crate::constants::MACRO_GENERATED_ERROR_TYPE;
use crate::constants::MULTIPLE_ERROR_TYPES;
use crate::constants::SOURCE_IN_ERROR_MESSAGE;
use crate::constants::UNTYPED_RESULT_ERROR;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorIssue {
    DiscardedError,
    ErrorSourceOutsideChain,
    ErrorTypeNotAnEnum,
    HandwrittenErrorType,
    MacroGeneratedErrorType,
    MultipleErrorTypes,
    SourceInErrorMessage,
    UntypedResultError,
}

impl ErrorIssue {
    #[must_use]
    pub const fn text(self) -> IssueText {
        match self {
            Self::DiscardedError => DISCARDED_ERROR,
            Self::ErrorSourceOutsideChain => ERROR_SOURCE_OUTSIDE_CHAIN,
            Self::ErrorTypeNotAnEnum => ERROR_TYPE_NOT_AN_ENUM,
            Self::HandwrittenErrorType => HANDWRITTEN_ERROR_TYPE,
            Self::MacroGeneratedErrorType => MACRO_GENERATED_ERROR_TYPE,
            Self::MultipleErrorTypes => MULTIPLE_ERROR_TYPES,
            Self::SourceInErrorMessage => SOURCE_IN_ERROR_MESSAGE,
            Self::UntypedResultError => UNTYPED_RESULT_ERROR,
        }
    }
}

impl From<&ErrorViolation> for ErrorIssue {
    fn from(violation: &ErrorViolation) -> Self {
        match violation {
            ErrorViolation::Discarded => Self::DiscardedError,
            ErrorViolation::Handwritten => Self::HandwrittenErrorType,
            ErrorViolation::MacroGenerated => Self::MacroGeneratedErrorType,
            ErrorViolation::MultipleTypes { .. } => Self::MultipleErrorTypes,
            ErrorViolation::NotAnEnum => Self::ErrorTypeNotAnEnum,
            ErrorViolation::SourceInMessage => Self::SourceInErrorMessage,
            ErrorViolation::SourceOutsideChain => Self::ErrorSourceOutsideChain,
            ErrorViolation::UntypedResult { .. } => Self::UntypedResultError,
        }
    }
}

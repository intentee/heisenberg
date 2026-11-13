use crate::constants::DISCARDED_ERROR;
use crate::constants::ERROR_SOURCE_OUTSIDE_CHAIN;
use crate::constants::HANDWRITTEN_ERROR_TYPE;
use crate::constants::MULTIPLE_ERROR_TYPES;
use crate::constants::SOURCE_IN_ERROR_MESSAGE;
use crate::constants::UNTYPED_RESULT_ERROR;
use crate::issue_text::IssueText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorIssue {
    DiscardedError,
    ErrorSourceOutsideChain,
    HandwrittenErrorType,
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
            Self::HandwrittenErrorType => HANDWRITTEN_ERROR_TYPE,
            Self::MultipleErrorTypes => MULTIPLE_ERROR_TYPES,
            Self::SourceInErrorMessage => SOURCE_IN_ERROR_MESSAGE,
            Self::UntypedResultError => UNTYPED_RESULT_ERROR,
        }
    }
}

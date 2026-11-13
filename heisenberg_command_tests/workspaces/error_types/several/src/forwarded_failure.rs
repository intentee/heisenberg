use std::io;
use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum ForwardedFailure {
    Read(io::Error),
    #[error("the text is empty: {0}")]
    Empty(ParseIntError),
}

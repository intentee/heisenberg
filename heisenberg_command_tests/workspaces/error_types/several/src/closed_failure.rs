use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ClosedFailure {
    Closed,
}

impl Display for ClosedFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("the channel is closed")
    }
}

impl Error for ClosedFailure {}

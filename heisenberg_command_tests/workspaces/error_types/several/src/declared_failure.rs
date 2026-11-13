use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

macro_rules! error_type {
    ($name:ident) => {
        impl Error for $name {}
    };
}

error_type!(DeclaredFailure);

#[derive(Debug)]
pub enum DeclaredFailure {
    Lost,
}

impl Display for DeclaredFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("the message is lost")
    }
}

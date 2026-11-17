use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, thiserror::Error)]
pub enum WrittenFailure {
    Missing,
}

impl Display for WrittenFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("the key is missing")
    }
}

use std::fmt;
use std::fmt::Formatter;

fn missing(formatter: &mut Formatter<'_>) -> fmt::Result {
    formatter.write_str("the key is missing")
}

#[derive(Debug, thiserror::Error)]
pub enum FormattedFailure {
    #[error(fmt = missing)]
    Missing,
}

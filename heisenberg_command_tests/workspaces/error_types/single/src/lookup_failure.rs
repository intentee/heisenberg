use std::io;
use std::num::ParseIntError;

const LIMIT: u8 = 3;

#[derive(Debug, thiserror::Error)]
pub enum LookupFailure {
    #[error("the key is missing")]
    Missing,
    #[error("the key cannot be read")]
    Unreadable(#[from] io::Error),
    #[error("the key {key} cannot be parsed")]
    Unparsable {
        key: String,
        #[source]
        source: ParseIntError,
    },
    #[error("the value cannot be parsed: {source}")]
    Value { source: ParseIntError },
    #[error("the limit is exceeded: {0:?}")]
    Exceeded(#[source] ParseIntError),
    #[error("the entry cannot be parsed: {}", &.source)]
    Entry { source: ParseIntError },
    #[error("the field cannot be parsed: {}", (.source))]
    Field { source: ParseIntError },
    #[error("the record has {} characters", .source.to_string().len())]
    Record { source: ParseIntError },
    #[error("the line is beyond {}", LIMIT)]
    Line { source: ParseIntError },
    #[error("the column {} cannot be parsed", .position.saturating_add(1))]
    Column {
        position: u8,
        #[source]
        source: ParseIntError,
    },
    #[error("the depth is beyond {}", 2)]
    Depth { source: ParseIntError },
    #[error("the limit {LIMIT} is reached")]
    Limit { source: ParseIntError },
    #[error(transparent)]
    Wrapped(ParseIntError),
    #[error("the pair cannot be parsed")]
    Pair {
        first: ParseIntError,
        #[source]
        second: ParseIntError,
    },
    #[error("the context is lost")]
    Context { context: io::Error },
}

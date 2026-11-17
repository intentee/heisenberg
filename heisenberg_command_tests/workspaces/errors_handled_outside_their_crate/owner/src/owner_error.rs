use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum OwnerError {
    #[error("the entry is missing")]
    Missing,
    #[error("the entry {0} is rejected")]
    Rejected(u8),
    #[error("the entry {key} cannot be parsed")]
    Unparsable {
        key: String,
        #[source]
        source: ParseIntError,
    },
}

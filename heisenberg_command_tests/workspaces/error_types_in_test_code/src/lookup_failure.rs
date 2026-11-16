#[derive(Debug, thiserror::Error)]
pub enum LookupFailure {
    #[error("the key is missing")]
    Missing,
}

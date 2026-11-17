#[derive(Debug, thiserror::Error)]
#[error("the lookup failed")]
pub enum FallbackFailure {
    Missing,
    #[error("the key cannot be read")]
    Unreadable,
}

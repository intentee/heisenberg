#[derive(Debug, thiserror::Error)]
pub enum RootFailure {
    #[error("the key is missing")]
    Missing,
}

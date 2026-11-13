#[derive(Debug, thiserror::Error)]
#[error("the file cannot be read")]
pub struct ReadFailure;

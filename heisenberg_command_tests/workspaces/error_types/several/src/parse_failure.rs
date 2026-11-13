#[derive(Debug, thiserror::Error)]
pub enum ParseFailure {
    #[error("the text is empty")]
    Empty,
}

pub mod inline_failure {
    #[derive(Debug, thiserror::Error)]
    pub enum InlineFailure {
        #[error("the key is missing")]
        Missing,
    }
}

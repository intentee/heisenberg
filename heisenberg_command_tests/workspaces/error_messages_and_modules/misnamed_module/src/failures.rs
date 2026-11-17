#[derive(Debug, thiserror::Error)]
enum LookupFailure {
    #[error("the key is missing")]
    Missing,
}

pub struct Failures;

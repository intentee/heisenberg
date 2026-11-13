pub trait IntoErrors<TValue, TError> {
    fn into_errors(self) -> Vec<Result<TValue, TError>>;
}

impl<TValue, TError> IntoErrors<TValue, TError> for Result<TValue, TError> {
    fn into_errors(self) -> Vec<Result<TValue, TError>> {
        vec![self]
    }
}

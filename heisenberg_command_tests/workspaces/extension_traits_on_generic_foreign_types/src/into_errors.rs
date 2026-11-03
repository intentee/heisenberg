pub trait IntoErrors<TValue, TError> {
    fn into_errors(self) -> Result<TValue, Vec<TError>>;
}

impl<TValue, TError> IntoErrors<TValue, TError> for Result<TValue, TError> {
    fn into_errors(self) -> Result<TValue, Vec<TError>> {
        self.map_err(|error| vec![error])
    }
}

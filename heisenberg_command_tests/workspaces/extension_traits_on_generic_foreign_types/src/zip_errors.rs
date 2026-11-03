pub trait ZipErrors<TValue, TError> {
    fn zip_errors(self) -> Vec<TError>;
}

impl<TValue, TError, TErrors: IntoIterator<Item = TError>> ZipErrors<TValue, TError>
    for Result<TValue, TErrors>
{
    fn zip_errors(self) -> Vec<TError> {
        self.err().into_iter().flatten().collect()
    }
}

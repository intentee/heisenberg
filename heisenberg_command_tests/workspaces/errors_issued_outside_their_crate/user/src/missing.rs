use owner::owner_error::OwnerError;

#[must_use]
pub const fn missing() -> OwnerError {
    OwnerError::Missing
}

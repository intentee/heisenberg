use owner::owner_error::OwnerError;

#[must_use]
pub fn received(result: &Result<u8, OwnerError>) -> u8 {
    result.as_ref().copied().unwrap_or_default()
}

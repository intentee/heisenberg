use owner::owner_error::OwnerError;

#[must_use]
pub const fn rejected(entry: u8) -> OwnerError {
    OwnerError::Rejected(entry)
}

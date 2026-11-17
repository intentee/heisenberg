use owner::owner_error::OwnerError;
use owner::parsed::parsed;

#[must_use]
pub fn recovered(text: &str) -> u8 {
    match parsed(text) {
        Ok(value) => value,
        Err(OwnerError::Missing) => 0,
        Err(OwnerError::Rejected(entry)) => entry,
        Err(OwnerError::Unparsable { .. }) => u8::MAX,
    }
}

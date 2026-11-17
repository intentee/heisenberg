use owner::owner_error::OwnerError;
use owner::parsed::parsed;

pub fn forwarded(text: &str) -> Result<u8, OwnerError> {
    parsed(text)
}

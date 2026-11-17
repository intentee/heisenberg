use crate::owner_error::OwnerError;
use crate::parsed::parsed;

pub fn converted<TError: From<OwnerError>>(text: &str) -> Result<u8, TError> {
    parsed(text).map_err(TError::from)
}

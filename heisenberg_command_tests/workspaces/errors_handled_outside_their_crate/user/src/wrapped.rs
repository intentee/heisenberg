use owner::parsed::parsed;

use crate::user_error::UserError;

pub fn wrapped(text: &str) -> Result<u8, UserError> {
    Ok(parsed(text)?)
}

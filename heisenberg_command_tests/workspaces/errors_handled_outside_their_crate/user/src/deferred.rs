use owner::owner_error::OwnerError;
use owner::parsed::parsed;

use crate::user_error::UserError;

pub fn deferred(text: &str) -> Result<u8, UserError> {
    let parse = |entry: &str| -> Result<u8, OwnerError> { parsed(entry) };

    Ok(parse(text)?)
}

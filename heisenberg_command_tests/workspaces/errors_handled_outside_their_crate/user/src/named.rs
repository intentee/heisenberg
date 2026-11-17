use owner::parsed::parsed;

use crate::user_error::UserError;

pub fn named(text: &str) -> Result<u8, UserError> {
    parsed(text).map_err(|source| UserError::Entry {
        entry: text.to_owned(),
        source,
    })
}

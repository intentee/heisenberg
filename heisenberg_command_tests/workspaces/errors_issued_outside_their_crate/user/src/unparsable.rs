use std::num::ParseIntError;

use owner::owner_error::OwnerError;

#[must_use]
pub const fn unparsable(key: String, source: ParseIntError) -> OwnerError {
    OwnerError::Unparsable { key, source }
}

use crate::owner_error::OwnerError;

pub fn parsed(text: &str) -> Result<u8, OwnerError> {
    text.parse().map_err(|source| OwnerError::Unparsable {
        key: text.to_owned(),
        source,
    })
}

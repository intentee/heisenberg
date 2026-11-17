use owner::owner_error::OwnerError;
use owner::parsed::parsed;

pub fn collected(text: &str) -> Result<u8, Vec<OwnerError>> {
    parsed(text).map_err(|failure| vec![failure])
}

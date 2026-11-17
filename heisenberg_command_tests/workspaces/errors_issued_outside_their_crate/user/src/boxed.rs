use owner::owner_error::OwnerError;
use owner::parsed::parsed;

pub fn boxed(text: &str) -> Result<u8, Box<OwnerError>> {
    parsed(text).map_err(Box::new)
}

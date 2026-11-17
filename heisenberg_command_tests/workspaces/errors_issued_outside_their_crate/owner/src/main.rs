use owner::owner_error::OwnerError;
use owner::parsed::parsed;

fn main() -> Result<(), OwnerError> {
    parsed("1").map(drop)
}

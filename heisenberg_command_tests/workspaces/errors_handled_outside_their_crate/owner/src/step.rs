use crate::owner_error::OwnerError;

pub trait Step {
    fn run(&self) -> Result<u8, OwnerError>;
}

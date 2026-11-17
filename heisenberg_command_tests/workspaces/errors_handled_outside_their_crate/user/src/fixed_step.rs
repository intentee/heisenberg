use owner::owner_error::OwnerError;
use owner::parsed::parsed;
use owner::step::Step;

pub struct FixedStep;

impl Step for FixedStep {
    fn run(&self) -> Result<u8, OwnerError> {
        parsed("1")
    }
}

use std::error::Error;

pub fn boxed_failure(succeeds: bool) -> Result<u8, Box<dyn Error>> {
    if succeeds { Ok(1) } else { Err("failed".into()) }
}

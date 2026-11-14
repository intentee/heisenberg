use std::num::ParseIntError;

pub fn inspected_result(text: &str) -> Result<u8, ParseIntError> {
    text.parse::<u8>().inspect(|_| ())
}

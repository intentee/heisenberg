use std::num::ParseIntError;

pub fn parse_digit(text: &str) -> Result<u8, ParseIntError> {
    text.parse()
}

use std::num::ParseIntError;

#[must_use]
pub fn error_bound_beside_rest(parsed: Result<u8, ParseIntError>) -> String {
    match parsed {
        Ok(value) => value.to_string(),
        Err(error, ..) => error.to_string(),
    }
}

use std::num::ParseIntError;

#[must_use]
pub fn flattened(parsed: Vec<Result<u8, ParseIntError>>) -> usize {
    parsed.into_iter().flatten().count()
}

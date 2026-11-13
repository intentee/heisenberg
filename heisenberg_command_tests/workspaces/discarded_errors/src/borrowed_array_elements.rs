use std::num::ParseIntError;

#[must_use]
pub const fn borrowed_array_elements(
    parsed: &[Result<u8, ParseIntError>; 2],
) -> &Result<u8, ParseIntError> {
    let [first, ..] = parsed;

    first
}

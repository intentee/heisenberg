use std::num::ParseIntError;

pub fn omitted_array_elements(
    parsed: [Result<u8, ParseIntError>; 2],
) -> Result<u8, ParseIntError> {
    let [first, ..] = parsed;

    first
}

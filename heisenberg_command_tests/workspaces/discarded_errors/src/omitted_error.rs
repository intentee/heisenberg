#[must_use]
pub fn omitted_error(text: &str) -> u8 {
    match text.parse::<u8>() {
        Ok(value) => value,
        Err(..) => 0,
    }
}

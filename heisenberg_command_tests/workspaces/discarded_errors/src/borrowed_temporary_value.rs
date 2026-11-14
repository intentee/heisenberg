#[must_use]
pub fn borrowed_temporary_value(text: &str) -> u8 {
    if let Ok(value) = &text.parse::<u8>() {
        *value
    } else {
        0
    }
}

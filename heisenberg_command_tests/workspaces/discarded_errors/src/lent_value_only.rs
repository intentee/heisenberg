#[must_use]
pub fn lent_value_only(text: &str) -> u8 {
    if let Ok(value) = text.parse::<u8>().as_ref() {
        *value
    } else {
        0
    }
}

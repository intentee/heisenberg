#[must_use]
pub fn value_only_if_let(text: &str) -> u8 {
    if let Ok(value) = text.parse::<u8>() {
        value
    } else {
        0
    }
}

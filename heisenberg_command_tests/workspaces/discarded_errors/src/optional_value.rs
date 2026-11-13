#[must_use]
pub fn optional_value(text: &str) -> Option<u8> {
    text.parse::<u8>().ok()
}

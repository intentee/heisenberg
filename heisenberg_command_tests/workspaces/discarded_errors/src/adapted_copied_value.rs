#[must_use]
pub fn adapted_copied_value(text: &str) -> Option<u8> {
    text.parse::<u8>().as_ref().copied().ok()
}

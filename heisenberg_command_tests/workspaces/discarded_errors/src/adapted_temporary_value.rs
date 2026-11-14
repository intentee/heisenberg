#[must_use]
pub fn adapted_temporary_value(text: &str) -> Option<u8> {
    text.parse::<u8>().as_ref().ok().copied()
}

#[must_use]
pub fn adapted_temporary_check(text: &str) -> bool {
    text.parse::<u8>().as_ref().is_ok()
}

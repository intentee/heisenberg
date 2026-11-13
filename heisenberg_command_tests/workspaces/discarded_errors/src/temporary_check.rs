#[must_use]
pub fn temporary_check(text: &str) -> bool {
    text.parse::<u8>().is_ok()
}

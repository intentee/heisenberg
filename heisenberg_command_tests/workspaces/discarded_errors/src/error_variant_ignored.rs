#[must_use]
pub fn error_variant_ignored(text: &str) -> u8 {
    match text.parse::<u8>() {
        Ok(value) => value,
        Err(_) => 0,
    }
}

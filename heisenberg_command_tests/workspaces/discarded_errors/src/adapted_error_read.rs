#[must_use]
pub fn adapted_error_read(text: &str) -> String {
    text.parse::<u8>()
        .as_ref()
        .map_or_else(ToString::to_string, ToString::to_string)
}

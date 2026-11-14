#[must_use]
pub fn stored_adapted_value(text: &str) -> Option<u8> {
    let parsed = text.parse::<u8>();
    let value = parsed.as_ref().ok().copied();

    match parsed {
        Ok(_) => value,
        Err(error) => Some(u8::from(error.to_string().is_empty())),
    }
}

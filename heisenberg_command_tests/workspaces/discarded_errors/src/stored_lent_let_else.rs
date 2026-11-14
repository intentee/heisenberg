#[must_use]
pub fn stored_lent_let_else(text: &str) -> u8 {
    let parsed = text.parse::<u8>();
    let Ok(value) = parsed.as_ref() else {
        return parsed.map_or_else(|error| u8::from(error.to_string().is_empty()), |value| value);
    };

    *value
}

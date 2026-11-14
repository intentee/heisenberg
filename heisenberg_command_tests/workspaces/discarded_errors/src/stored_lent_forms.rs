#[must_use]
pub fn stored_lent_forms(text: &str) -> u8 {
    let parsed = text.parse::<u8>();
    let mut last = 0;

    for value in parsed.as_ref() {
        last = *value;
    }

    let _ = parsed.as_ref();
    _ = parsed.as_ref();

    let borrowed = if let Ok(value) = parsed.as_ref() {
        *value
    } else {
        0
    };

    match parsed {
        Ok(value) => value.saturating_add(last).saturating_add(borrowed),
        Err(error) => u8::from(error.to_string().is_empty()),
    }
}

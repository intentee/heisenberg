use std::convert::Infallible;

use crate::attempt::Attempt;

#[must_use]
pub fn errors_read_or_exempt(text: &str) -> u8 {
    let parsed = text.parse::<u8>();
    let borrowed: Result<u8, &str> = Err(text);
    let _ = Ok::<u8, ()>(1);
    let _ = Ok::<u8, Infallible>(1);
    let _ = borrowed;
    let reference = &parsed;
    let attempt = Attempt {
        parsed: text.parse::<u8>(),
        value: 1,
    };
    let results = [text.parse::<u8>()];
    let checked = parsed.is_ok()
        && (*reference).is_ok()
        && attempt.parsed.is_ok()
        && results[0].is_ok();
    let from_a_reference = if let Ok(value) = &parsed { *value } else { 0 };

    match parsed {
        Ok(value) if checked => value.saturating_add(from_a_reference),
        Ok(value) => value,
        Err(error) => u8::from(error.to_string().is_empty()),
    }
}

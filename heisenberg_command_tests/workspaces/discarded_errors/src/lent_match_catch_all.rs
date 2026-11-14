#[must_use]
pub fn lent_match_catch_all(text: &str) -> u8 {
    match text.parse::<u8>().as_ref() {
        Ok(value) => *value,
        _ => 0,
    }
}

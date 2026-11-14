#[must_use]
pub fn lent_match_rest(text: &str) -> u8 {
    match text.parse::<u8>().as_ref() {
        Ok(value) => *value,
        Err(..) => 0,
    }
}

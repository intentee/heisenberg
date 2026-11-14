#[must_use]
pub fn lent_match_arm(text: &str) -> u8 {
    match text.parse::<u8>().as_ref() {
        Ok(value) => *value,
        Err(_) => 0,
    }
}

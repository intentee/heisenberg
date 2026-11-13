#[must_use]
pub fn value_only_let_else(text: &str) -> u8 {
    let Ok(value) = text.parse::<u8>() else {
        return 0;
    };

    value
}

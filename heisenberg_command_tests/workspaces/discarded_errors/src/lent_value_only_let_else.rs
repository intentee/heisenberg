#[must_use]
pub fn lent_value_only_let_else(text: &str) -> u8 {
    let Ok(value) = text.parse::<u8>().as_ref().copied() else {
        return 0;
    };

    value
}

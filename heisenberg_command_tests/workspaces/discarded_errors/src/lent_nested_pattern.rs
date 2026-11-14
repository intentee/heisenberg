#[must_use]
pub fn lent_nested_pattern(text: &str) -> u8 {
    let (_, value) = (text.parse::<u8>().as_ref(), 1);

    value
}

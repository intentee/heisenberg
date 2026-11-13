#[must_use]
pub fn omitted_tuple_element(text: &str) -> u8 {
    let (value, ..) = (1, text.parse::<u8>());

    value
}

#[must_use]
pub fn function_value(texts: &[&str]) -> usize {
    texts
        .iter()
        .map(|text| text.parse::<u8>())
        .filter_map(Result::ok)
        .count()
}

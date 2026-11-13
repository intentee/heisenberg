#[must_use]
pub fn value_iterator(text: &str) -> usize {
    text.parse::<u8>().into_iter().count()
}

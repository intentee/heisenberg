#[must_use]
pub fn borrowed_into_iterator(text: &str) -> usize {
    (&text.parse::<u8>()).into_iter().count()
}

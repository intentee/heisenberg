#[must_use]
pub fn mutable_iterator(text: &str) -> usize {
    text.parse::<u8>().iter_mut().count()
}

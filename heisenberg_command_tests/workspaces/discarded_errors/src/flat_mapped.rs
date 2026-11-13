#[must_use]
pub fn flat_mapped(texts: &[&str]) -> usize {
    texts.iter().flat_map(|text| text.parse::<u8>()).count()
}

pub fn total(values: &[u8]) -> usize {
    values
        .iter()
        .enumerate()
        .map(|entry| entry.0)
        .sum()
}

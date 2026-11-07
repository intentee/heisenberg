pub fn total() -> u8 {
    [provider::current()]
        .iter()
        .map(|reading| reading.value)
        .sum()
}

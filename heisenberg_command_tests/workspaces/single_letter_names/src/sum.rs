pub fn sum(x: u8, values: &[u8]) -> u8 {
    values.iter().fold(x, |total, &v| total + v) + values.iter().map(|y| y + 1).sum::<u8>()
}

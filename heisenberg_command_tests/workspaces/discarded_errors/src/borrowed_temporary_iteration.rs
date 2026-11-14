#[must_use]
pub fn borrowed_temporary_iteration(text: &str) -> u8 {
    let mut last = 0;

    for value in &text.parse::<u8>() {
        last = *value;
    }

    last
}

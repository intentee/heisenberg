#[must_use]
pub fn iterated_result(text: &str) -> u8 {
    let mut last = 0;

    for value in text.parse::<u8>() {
        last = value;
    }

    last
}

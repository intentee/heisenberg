#[must_use]
pub fn lent_iteration(text: &str) -> u8 {
    let mut last = 0;

    for value in text.parse::<u8>().as_ref() {
        last = *value;
    }

    last
}

#[must_use]
pub fn parenthesized_lender(text: &str) -> Option<u8> {
    (text.parse::<u8>().as_ref()).ok().copied()
}

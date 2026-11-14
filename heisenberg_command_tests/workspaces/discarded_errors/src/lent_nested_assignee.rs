#[must_use]
pub fn lent_nested_assignee(text: &str) -> u8 {
    let value;

    (_, value) = (text.parse::<u8>().as_ref(), 1);

    value
}

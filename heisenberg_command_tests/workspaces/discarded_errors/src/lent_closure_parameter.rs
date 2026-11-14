#[must_use]
pub fn lent_closure_parameter(text: &str) -> u8 {
    *text.parse::<u8>().as_ref().unwrap_or_else(|_| &0)
}

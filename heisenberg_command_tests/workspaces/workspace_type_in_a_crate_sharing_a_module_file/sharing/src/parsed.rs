use types::rejection::Rejection;

pub fn parsed(text: &str) -> Result<usize, Rejection> {
    text.parse().map_err(|source| Rejection { source })
}

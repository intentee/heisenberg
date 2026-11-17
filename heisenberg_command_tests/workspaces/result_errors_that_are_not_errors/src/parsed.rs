use crate::rejection::Rejection;

pub fn parsed(text: &str) -> Result<usize, Rejection> {
    match text.len() {
        0 => Err(Rejection::Empty),
        length if length > 8 => Err(Rejection::TooLong),
        length => Ok(length),
    }
}

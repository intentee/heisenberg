use crate::rejection::Rejection;

pub type Decision = Result<usize, Rejection>;

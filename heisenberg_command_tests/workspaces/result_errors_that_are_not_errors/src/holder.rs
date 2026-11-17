use crate::rejection::Rejection;

pub struct Holder {
    pub result: Result<usize, Rejection>,
}

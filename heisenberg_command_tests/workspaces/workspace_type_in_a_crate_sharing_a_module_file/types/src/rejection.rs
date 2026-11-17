use std::num::ParseIntError;

pub struct Rejection {
    pub source: ParseIntError,
}

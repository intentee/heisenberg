pub const LIMIT: u8 = 3;

pub const DOUBLE: u8 = double(LIMIT);

pub struct Unit;

pub fn helper() -> u8 {
    LIMIT
}

pub static COUNTER: u8 = 0;

const fn double(value: u8) -> u8 {
    value * 2
}

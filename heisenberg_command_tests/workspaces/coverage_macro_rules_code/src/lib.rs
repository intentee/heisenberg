macro_rules! scaled {
    ($factor:expr) => {
        |value: u32| value * $factor
    };
}

pub static QUADRUPLE: fn(u32) -> u32 = scaled!(4);

pub fn quintuple(value: u32) -> u32 {
    value * 5
}

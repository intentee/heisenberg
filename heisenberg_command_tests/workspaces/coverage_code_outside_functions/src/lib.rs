pub static DOUBLE: fn(u32) -> u32 = |value| value * 2;

pub static PARITY: fn(u32) -> &'static str = |value| if value % 2 == 0 { "even" } else { "odd" };

pub fn triple(value: u32) -> u32 {
    value * 3
}

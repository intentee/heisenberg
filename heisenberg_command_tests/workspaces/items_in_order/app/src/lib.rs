pub mod constants;

extern crate alloc;

use std::fmt::Display;

use shapes::square::Square;

use crate::constants::UNIT;

const SCALE: u32 = 2;

unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

thread_local! {
    static COUNT: u8 = const { 0 };
}

fn scaled(square: &Square) -> u32 {
    square.area() * SCALE + abs(0).unsigned_abs()
}

struct Wrapper;

impl Wrapper {
    fn unit() -> &'static str {
        UNIT
    }
}

pub fn describe(square: &Square) -> impl Display {
    format!("{}{}", scaled(square), Wrapper::unit())
}

#[cfg(test)]
mod tests {
    #[test]
    fn describes() { assert!(!"describes".is_empty()); }
}

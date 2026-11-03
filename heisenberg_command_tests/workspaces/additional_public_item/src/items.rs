pub struct Items;

pub fn second() {}

pub static THIRD: u8 = 3;

#[macro_export]
macro_rules! fourth {
    () => {};
}

macro_rules! private_macro {
    () => {};
}

fn private_helper() {}

pub mod nested {
    pub struct Nested;

    pub enum AlsoNested {}
}

pub type Fifth = u8;

pub macro sixth() {}

unsafe extern "C" {
    pub safe fn abs(value: i32) -> i32;
}

pub union Seventh {
    value: u8,
}

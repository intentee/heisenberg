pub struct First;

pub fn second() {}

pub const THIRD: u8 = 3;

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

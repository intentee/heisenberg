const PRIVATE: u8 = 4;

pub const FIRST: u8 = 1;

pub const SECOND: u8 = 2;

pub(crate) const THIRD: u8 = 3;

pub const FOURTH: u8 = PRIVATE;

pub mod nested {
    pub const DEEP: u8 = 5;
}

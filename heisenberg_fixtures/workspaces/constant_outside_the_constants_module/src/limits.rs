use crate::bounded::Bounded;

pub const MAXIMUM: u8 = 9;

pub(crate) const SHARED: u8 = 4;

pub struct Limits;

impl Limits {
    pub const MINIMUM: u8 = 1;

    const STEP: u8 = 2;

    pub fn total() -> u8 {
        pub const INNER: u8 = 3;

        INNER + Self::STEP
    }
}

impl Bounded for Limits {
    const UPPER: u8 = 9;
}

use std::num::ParseIntError;

pub struct Attempt {
    pub parsed: Result<u8, ParseIntError>,
    pub value: u8,
}

impl Attempt {
    #[must_use]
    pub fn value(self) -> u8 {
        let Self { value, .. } = self;

        value
    }
}

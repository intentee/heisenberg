use std::num::ParseIntError;

pub struct OmittedField(pub u8, pub Result<u8, ParseIntError>);

impl OmittedField {
    #[must_use]
    pub const fn borrowed_value(&self) -> u8 {
        let Self(value, ..) = self;

        *value
    }

    #[must_use]
    pub fn value(self) -> u8 {
        let Self(value, ..) = self;

        value
    }
}

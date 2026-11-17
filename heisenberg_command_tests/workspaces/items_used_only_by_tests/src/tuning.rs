use crate::constants::LIMIT;

pub struct Tuning {
    pub gain: u8,
    pub width: u8,
}

impl Tuning {
    pub const fn amplified(self) -> Self {
        type Tuning = crate::tuning::Tuning;

        Tuning {
            gain: LIMIT,
            ..self
        }
    }
}

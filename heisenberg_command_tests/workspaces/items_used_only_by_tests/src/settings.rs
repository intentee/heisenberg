use crate::constants::LIMIT;

pub struct Settings {
    pub depth: u8,
    pub width: u8,
}

impl Settings {
    pub const fn deepened(self) -> Self {
        Self {
            depth: LIMIT,
            ..self
        }
    }
}

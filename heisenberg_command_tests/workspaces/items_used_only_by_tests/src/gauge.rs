use crate::constants::LIMIT;
use crate::level::Level;

pub struct Gauge {
    pub level: u8,
    pub observed: u8,
}

impl Gauge {
    pub const fn new() -> Self {
        Self {
            level: LIMIT,
            observed: 0,
        }
    }

    pub const fn level(&self) -> Level {
        if self.level > LIMIT {
            Level::High
        } else {
            Level::Low
        }
    }
}

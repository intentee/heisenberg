pub enum Signal {
    Pulse { strength: u8, width: u8 },
}

impl Signal {
    pub const fn width(&self) -> u8 {
        match self {
            Self::Pulse { width, .. } => *width,
        }
    }
}

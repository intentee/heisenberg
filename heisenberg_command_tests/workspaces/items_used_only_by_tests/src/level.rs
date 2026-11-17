pub enum Level {
    High,
    Low,
    Probed,
}

impl Level {
    pub const fn is_high(&self) -> bool {
        match self {
            Self::High => true,
            Self::Low | Self::Probed => false,
        }
    }
}

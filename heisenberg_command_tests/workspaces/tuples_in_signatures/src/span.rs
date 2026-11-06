pub struct Span {
    pub bounds: (u8, u8),
}

impl From<(u8, u8)> for Span {
    fn from(bounds: (u8, u8)) -> Self {
        Self { bounds }
    }
}

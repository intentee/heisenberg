pub struct Report {
    pub lines: u8,
    pub pages: u8,
}

impl Report {
    #[must_use]
    pub const fn doubled(&self) -> u8 {
        self.lines.saturating_mul(2).saturating_add(self.pages)
    }

    #[must_use]
    pub const fn tripled(&self) -> u8 {
        self.lines.saturating_mul(3)
    }
}

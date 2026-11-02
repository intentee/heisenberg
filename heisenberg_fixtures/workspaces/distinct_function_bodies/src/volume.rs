pub struct Volume {
    pub depth: u8,
}

impl Volume {
    pub const fn is_empty(&self) -> bool {
        false
    }
}

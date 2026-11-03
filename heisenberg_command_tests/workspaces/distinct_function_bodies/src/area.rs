pub struct Area {
    pub width: u8,
    pub height: u8,
}

impl Area {
    pub const fn width(&self) -> u8 {
        self.width
    }

    pub const fn height(&self) -> u8 {
        self.height
    }

    pub const fn doubled_width(&self) -> u8 {
        self.width() * 2
    }

    pub const fn doubled_height(&self) -> u8 {
        self.height() * 2
    }

    pub const fn is_empty(&self) -> bool {
        false
    }
}

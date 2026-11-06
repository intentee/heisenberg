pub struct Square {
    side: u32,
}

impl Square {
    pub fn new(side: u32) -> Self {
        Self { side }
    }

    pub fn area(&self) -> u32 {
        self.side * self.side
    }
}

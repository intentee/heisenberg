pub struct Counter {
    pub value: u8,
}

impl Counter {
    pub const fn new() -> Self {
        Self { value: 0 }
    }

    pub const fn for_tests() -> Self {
        Self { value: 7 }
    }

    pub const fn increment(&mut self) {
        self.value += 1;
    }

    pub const fn unused(&self) -> u8 {
        self.value
    }
}

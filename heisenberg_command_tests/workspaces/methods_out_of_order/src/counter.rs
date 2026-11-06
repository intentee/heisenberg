pub struct Counter {
    count: u8,
}

impl Counter {
    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn new() -> Self {
        Self { count: 0 }
    }

    fn reset(&mut self) {
        self.count = 0;
    }

    pub(super) fn halve(&mut self) {
        self.count /= 2;
    }

    pub(crate) fn bump(&mut self) {
        self.count += 1;
    }

    pub fn add(&mut self, amount: u8) {
        self.count += amount;
    }
}

impl Iterator for Counter {
    type Item = u8;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    fn next(&mut self) -> Option<u8> {
        None
    }
}

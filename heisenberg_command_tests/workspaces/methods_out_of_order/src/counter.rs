use crate::tally::Tally;

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

    pub fn reversed(&self) -> Self {
        Self {
            count: u8::MAX - self.count,
        }
    }

    pub(crate) fn empty() -> Self {
        Self { count: 1 }
    }

    pub fn with_count(self, count: u8) -> Self {
        Self { count, ..self }
    }

    pub(super) fn third() -> Self {
        Self { count: 3 }
    }

    fn fourth() -> Self {
        Self { count: 4 }
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

impl Tally for Counter {
    fn total(&self) -> u8 {
        self.count
    }

    fn create() -> Self {
        Self { count: 2 }
    }

    fn average(&self) -> u8 {
        self.count / 2
    }
}

use std::num::ParseIntError;

pub struct Counter {
    count: u8,
}

impl Counter {
    pub fn first(values: &[u8]) -> Option<Self> {
        values.first().map(|count| Self { count: *count })
    }

    pub async fn loaded() -> Self {
        Self { count: 1 }
    }

    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn parsed(text: &str) -> Result<Self, ParseIntError> {
        text.parse().map(|count| Self { count })
    }

    pub fn with_count(mut self, count: u8) -> Self {
        self.count = count;
        self
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub(crate) fn bump(&mut self) {
        self.count += 1;
    }

    pub(super) fn halve(&mut self) {
        self.count /= 2;
    }

    fn reset(&mut self) {
        self.count = 0;
    }
}

impl Iterator for Counter {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

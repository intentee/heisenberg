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

    const fn doubled(&self) -> u8 {
        self.value * 2
    }
}

struct Hidden;

impl Hidden {
    pub const fn value(&self) -> u8 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::Counter;
    use super::Hidden;

    #[test]
    fn counter_doubles() {
        assert_eq!(Counter::new().doubled(), 0);
    }

    #[test]
    fn hidden_has_a_value() {
        assert_eq!(Hidden.value(), 1);
    }
}

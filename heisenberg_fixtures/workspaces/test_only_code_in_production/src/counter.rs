pub struct Counter {
    pub value: u8,
}

impl Counter {
    pub const fn incremented(&self) -> u8 {
        #[cfg(test)]
        assert!(self.value < u8::MAX);

        self.value + 1
    }

    #[cfg(test)]
    pub const fn for_tests() -> Self {
        Self { value: 0 }
    }
}

#[cfg(test)]
impl Counter {
    pub const fn doubled(&self) -> u8 {
        self.value * 2
    }
}

#[cfg(test)]
const fn module_helper() -> u8 {
    1
}

#[cfg(test)]
mod checks {
    #[test]
    fn module_helper_returns_one() {
        assert_eq!(super::module_helper(), 1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn counter_increments() {
        assert_eq!(super::Counter::for_tests().doubled(), 0);
    }
}

#[cfg(test)]
use std::mem;

#[cfg(test)]
const fn module_helper() -> u8 {
    1
}

pub struct Counter {
    pub value: u8,
}

impl Counter {
    #[cfg(test)]
    pub const fn for_tests() -> Self {
        Self { value: 0 }
    }

    #[cfg_attr(test, must_use)]
    pub const fn attributed(&self) -> u8 {
        self.value
    }

    pub const fn incremented(&self) -> u8 {
        #[cfg(test)]
        assert!(self.value < u8::MAX);

        self.value + 1
    }

    #[cfg(any(test, feature = "probe"))]
    pub const fn probed(&self) -> u8 {
        self.value
    }

    pub const fn under_test(&self) -> bool {
        cfg!(any(all(feature = "probe", not(test)))) && self.value == 0
    }
}

#[cfg(test)]
impl Counter {
    pub const fn doubled(&self) -> u8 {
        self.value * 2
    }
}

#[cfg(test)]
mod tests {
    use super::Counter;
    use super::mem;
    use super::module_helper;

    #[cfg(test)]
    const fn nested_helper() -> u8 {
        module_helper()
    }

    #[test]
    fn counter_starts_at_zero() {
        assert_eq!(Counter::for_tests().doubled(), 0);
        assert_eq!(mem::size_of::<Counter>(), usize::from(nested_helper()));
    }
}

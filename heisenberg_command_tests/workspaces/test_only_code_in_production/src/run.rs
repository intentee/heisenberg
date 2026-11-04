use crate::counter::Counter;

pub const fn run() -> u8 {
    Counter { value: 1 }.incremented()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_increments() {
        assert_eq!(
            crate::counter::Counter {
                value: crate::sample_value::sample_value()
            }
            .incremented(),
            4
        );
    }
}

use crate::counter::Counter;

pub const fn run() -> u8 {
    Counter { value: 1 }.incremented()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn run_increments_one() {
        assert_eq!(run(), 2);
    }
}

pub const fn answer() -> u8 {
    42
}

#[cfg(test)]
mod tests {
    use super::answer;

    #[test]
    #[ignore]
    fn answer_is_forty_two() {
        assert_eq!(answer(), 42);
    }

    #[test]
    #[cfg_attr(test, ignore = "the answer is not checked")]
    fn answer_is_not_zero() {
        assert_ne!(answer(), 0);
    }

    #[test]
    fn answer_is_below_one_hundred() {
        assert!(answer() < 100);
    }
}

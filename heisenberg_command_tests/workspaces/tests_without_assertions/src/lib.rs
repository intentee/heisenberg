pub const fn double(value: u8) -> u8 {
    value.saturating_mul(2)
}

#[cfg(test)]
mod tests {
    use super::double;

    macro_rules! assert_doubled {
        ($value:expr, $doubled:expr) => {
            assert_eq!(double($value), $doubled)
        };
    }

    #[test]
    fn doubling_runs() {
        double(1);
    }

    #[test]
    fn doubling_is_asserted_through_a_macro() {
        assert_doubled!(2, 4);
    }

    #[test]
    #[should_panic(expected = "missing")]
    fn missing_value_panics() {
        Option::<u8>::None.expect("missing");
    }

    #[test]
    fn doubling_is_asserted_only_by_a_nested_function() {
        fn doubles_two() {
            assert_eq!(double(2), 4);
        }

        doubles_two();
    }
}

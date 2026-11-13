pub struct Calculator;

#[cfg(test)]
mod tests {
    struct Fixture;

    #[test]
    fn adds() {
        let _fixture = Fixture;
        assert!(!"adds".is_empty()); }

    mod helpers_after_tests {
        #[test]
        fn subtracts() {
            helper();
            assert!(!"subtracts".is_empty()); }

        fn helper() {}
    }
}

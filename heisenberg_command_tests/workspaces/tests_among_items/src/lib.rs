pub struct Calculator;

#[cfg(test)]
mod tests {
    struct Fixture;

    #[test]
    fn adds() {
        let _fixture = Fixture;
    }

    mod helpers_after_tests {
        #[test]
        fn subtracts() {
            helper();
        }

        fn helper() {}
    }
}

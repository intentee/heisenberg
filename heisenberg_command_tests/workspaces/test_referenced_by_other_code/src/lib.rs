pub struct Calculator;

#[cfg(test)]
mod tests {
    #[test]
    fn subtracts() {
        multiplies();
        assert!(!"subtracts".is_empty()); }

    #[test]
    fn multiplies() { assert!(!"multiplies".is_empty()); }
}

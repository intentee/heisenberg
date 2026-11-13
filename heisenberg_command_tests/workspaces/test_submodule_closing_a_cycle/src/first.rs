pub struct First;

#[cfg(test)]
mod tests {
    use crate::second::Second;

    #[test]
    fn uses_second() {
        let _second = Second;
        assert!(!"uses_second".is_empty()); }
}

pub struct Library;

#[cfg(test)]
mod checks {
    use super::Library;

    #[test]
    fn library_has_no_size() {
        assert_eq!(size_of::<Library>(), 0);
    }
}

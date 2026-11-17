pub struct Library;

#[cfg(test)]
mod tests {
    use super::Library;

    #[cfg(test)]
    const fn expected_size() -> usize {
        0
    }

    #[test]
    fn library_has_no_size() {
        assert_eq!(size_of::<Library>(), expected_size());
    }
}

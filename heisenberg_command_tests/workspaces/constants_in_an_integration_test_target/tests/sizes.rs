use constants_in_an_integration_test_target::Library;

const LIBRARY_SIZE: usize = 0;

#[test]
fn library_has_no_size() {
    assert_eq!(size_of::<Library>(), LIBRARY_SIZE);
}

mod units {
    const UNIT_SIZE: usize = 0;

    #[test]
    fn unit_has_no_size() {
        assert_eq!(size_of::<()>(), UNIT_SIZE);
    }
}

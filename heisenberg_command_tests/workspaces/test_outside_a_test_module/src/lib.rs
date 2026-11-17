pub struct Library;

#[test]
fn library_has_no_size() {
    assert_eq!(size_of::<Library>(), 0);
}

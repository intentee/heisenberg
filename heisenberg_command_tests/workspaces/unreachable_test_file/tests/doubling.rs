use unreachable_test_file::doubled::doubled;

#[test]
fn doubling_saturates() {
    assert_eq!(doubled(200), u8::MAX);
}

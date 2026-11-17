use unreachable_source_files::doubled::doubled;

#[test]
fn doubling_of_zero_is_zero() {
    assert_eq!(doubled(0), 0);
}

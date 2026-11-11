#[test]
fn production_only_is_one() {
    assert_eq!(coverage_production_only_code::production_only(), 1);
}

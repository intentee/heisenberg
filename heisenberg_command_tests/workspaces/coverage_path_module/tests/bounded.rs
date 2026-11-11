#[test]
fn small_value_stays() {
    assert_eq!(coverage_path_module::bounded(7), 7);
}

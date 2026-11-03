use functions_used_only_by_tests_in_a_publishable_library::counter::Counter;

#[test]
fn counter_for_tests_starts_at_seven() {
    assert_eq!(Counter::for_tests().value, 7);
}

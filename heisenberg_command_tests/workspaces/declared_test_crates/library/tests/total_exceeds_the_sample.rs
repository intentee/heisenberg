#[test]
fn total_exceeds_the_sample() {
    assert!(library::total() > fixtures::sample());
    assert_eq!(shared::for_tests::for_tests(), 3);
}

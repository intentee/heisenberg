use library::counter::Counter;

#[test]
fn counter_increments() {
    let mut counter = Counter::for_tests();

    counter.increment();

    assert_eq!(counter.value, 8);
}

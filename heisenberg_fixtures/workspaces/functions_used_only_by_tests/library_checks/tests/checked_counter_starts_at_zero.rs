use library_checks::checked_counter;

#[test]
fn checked_counter_starts_at_zero() {
    assert_eq!(checked_counter().value, 0);
}

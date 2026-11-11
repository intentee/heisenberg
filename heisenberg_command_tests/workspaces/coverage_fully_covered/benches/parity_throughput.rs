#[test]
fn parity_of_a_large_value() {
    assert_eq!(coverage_fully_covered::parity(1_000_000), "even");
}

#[test]
fn meter_clones() {
    let _ = coverage_without_coverage_data::Meter.clone();
}

#[test]
fn instrumented_returns_five() {
    assert_eq!(coverage_without_coverage_data::instrumented(), 5);
}

#[test]
fn twenty_is_above_ten() {
    assert_eq!(
        coverage_shared_macro_text::above_ten(20),
        "above"
    );
}

#[test]
fn five_is_within_ten() {
    assert_eq!(
        coverage_shared_macro_text::above_ten(5),
        "within"
    );
}

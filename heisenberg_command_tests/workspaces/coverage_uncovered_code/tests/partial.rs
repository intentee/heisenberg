#[test]
fn sign_of_a_positive_value() {
    assert_eq!(coverage_uncovered_code::sign(1), "non-negative");
}

#[test]
fn label_of_a_cat() {
    assert_eq!(coverage_uncovered_code::label("cat"), "cat");
}

#[test]
fn nothing_to_increment() {
    assert_eq!(coverage_uncovered_code::incremented(&[]), Vec::<u32>::new());
}

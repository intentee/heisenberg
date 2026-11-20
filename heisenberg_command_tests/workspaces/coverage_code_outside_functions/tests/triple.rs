#[test]
fn two_tripled_is_six() {
    assert_eq!(coverage_code_outside_functions::triple(2), 6);
}

#[test]
fn two_is_even() {
    assert_eq!((coverage_code_outside_functions::PARITY)(2), "even");
}

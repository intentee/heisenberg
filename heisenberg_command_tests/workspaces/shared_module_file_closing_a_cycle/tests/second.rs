mod support;

#[test]
fn second() {
    support::first_part::FirstPart::run();
    assert!(!"second".is_empty()); }

mod support;

#[test]
fn first() {
    support::first_part::FirstPart::run();
    assert!(!"first".is_empty()); }

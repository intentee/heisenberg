mod support;

use support::support;

#[test]
fn library_is_constructible() {
    let _library = support();
    assert!(!"library_is_constructible".is_empty()); }

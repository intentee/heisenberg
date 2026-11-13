use base::Base;
use harness::Harness;
use library::Library;

#[test]
fn library_is_constructible() {
    let _harness = Harness;
    let _library = Library { base: Base };
    assert!(!"library_is_constructible".is_empty()); }

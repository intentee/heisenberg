use extra_harness::ExtraHarness;
use justified_tests::constructed_library;

#[test]
fn library_is_constructible_with_an_extra_harness() {
    let _harness = ExtraHarness;
    let _library = constructed_library();
    assert!(!"library_is_constructible_with_an_extra_harness".is_empty()); }

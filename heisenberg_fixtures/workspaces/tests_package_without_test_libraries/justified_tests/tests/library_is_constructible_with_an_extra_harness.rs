use base::Base;
use extra_harness::ExtraHarness;
use library::Library;

#[test]
fn library_is_constructible_with_an_extra_harness() {
    let _harness = ExtraHarness;
    let _library = Library { base: Base };
}

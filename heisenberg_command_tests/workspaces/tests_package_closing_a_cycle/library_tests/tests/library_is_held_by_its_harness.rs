use harness::Harness;
use library::Library;

#[test]
fn library_is_held_by_its_harness() {
    let _harness = Harness { library: Library };
}

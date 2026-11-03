mod support;

use support::constructed_library;

#[test]
fn library_is_constructible() {
    let _library = constructed_library();
}

use non_test_item_in_test_target::Library;

fn root_helper() -> Library {
    Library
}

#[test]
fn builtin_test() {
    let _library = root_helper();
    let _helped = helpers::helpers();
    assert!(!"builtin_test".is_empty()); }

#[runtime::test]
fn attribute_macro_test() { assert!(!"attribute_macro_test".is_empty()); }

#[test]
#[ignore = "the scenario needs no run"]
fn ignored_test() { assert!(!"ignored_test".is_empty()); }

mod helpers {
    pub fn helpers() -> super::Library {
        super::Library
    }
}

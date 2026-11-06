use non_test_item_in_test_target::Library;

fn root_helper() -> Library {
    Library
}

#[test]
fn builtin_test() {
    let _library = root_helper();
    let _helped = helpers::helpers();
}

#[runtime::test]
fn attribute_macro_test() {}

#[test]
#[ignore = "the scenario needs no run"]
fn ignored_test() {}

mod helpers {
    pub fn helpers() -> super::Library {
        super::Library
    }
}

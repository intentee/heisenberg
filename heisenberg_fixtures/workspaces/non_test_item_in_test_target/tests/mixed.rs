use non_test_item_in_test_target::Library;

mod helpers {
    pub fn helpers() -> super::Library {
        super::Library
    }
}

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

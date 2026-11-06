use custom_harness::Library;

const TRIAL_NAME: &str = "library_is_constructible";

fn library_is_constructible() -> bool {
    let _library = Library;

    true
}

#[cfg(test)]
fn compiled_only_with_libtest() {}

fn main() {
    assert!(library_is_constructible(), "{TRIAL_NAME} failed");
    assert_eq!(shared_helper(), 1);
}

pub fn shared_helper() -> u8 {
    1
}

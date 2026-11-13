#[test]
fn runs_both_modules() {
    first::first();
    second::back();
    assert!(!"runs_both_modules".is_empty()); }

mod first {
    pub fn first() {
        super::second::second();
    }
}

mod second {
    pub fn second() {}

    pub fn back() {
        super::first::first();
    }
}

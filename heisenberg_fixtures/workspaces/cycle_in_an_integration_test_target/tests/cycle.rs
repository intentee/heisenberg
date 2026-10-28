mod first {
    pub fn run() {
        super::second::helper();
    }
}

mod second {
    pub fn helper() {}

    pub fn back() {
        super::first::run();
    }
}

#[test]
fn runs_both_modules() {
    first::run();
    second::back();
}

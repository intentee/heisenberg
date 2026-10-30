pub struct Second;

impl Second {
    pub fn back() {
        crate::first::run();
    }

    pub fn helper() {}
}

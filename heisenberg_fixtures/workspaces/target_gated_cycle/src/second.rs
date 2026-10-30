pub struct Second;

impl Second {
    #[cfg(windows)]
    pub fn back() {
        crate::first::run();
    }

    pub fn helper() {}
}

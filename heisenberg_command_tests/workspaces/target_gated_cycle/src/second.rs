pub struct Second;

impl Second {
    #[cfg(windows)]
    pub fn back() {
        crate::first::first();
    }

    pub fn helper() {}
}

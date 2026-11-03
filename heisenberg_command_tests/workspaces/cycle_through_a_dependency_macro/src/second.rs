pub struct Second;

impl Second {
    pub fn back() {
        crate::first::first();
    }

    pub fn helper() {}
}

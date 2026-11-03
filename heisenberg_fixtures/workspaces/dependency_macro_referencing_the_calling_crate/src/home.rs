pub struct Home;

impl Home {
    pub fn back() {
        crate::caller::caller();
    }

    pub fn run() {}
}

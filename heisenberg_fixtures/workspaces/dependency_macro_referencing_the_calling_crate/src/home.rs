pub struct Home;

impl Home {
    pub fn back() {
        crate::caller::call();
    }

    pub fn run() {}
}

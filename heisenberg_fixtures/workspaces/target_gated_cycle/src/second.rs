pub fn helper() {}

#[cfg(windows)]
pub fn back() {
    crate::first::run();
}

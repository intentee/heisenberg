#[cfg(windows)]
pub fn run() {
    windows_only::call!(crate::second::Second::helper);
}

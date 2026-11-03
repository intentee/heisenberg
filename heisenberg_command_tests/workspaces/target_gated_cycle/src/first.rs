#[cfg(windows)]
pub fn first() {
    windows_only::call!(crate::second::Second::helper);
}

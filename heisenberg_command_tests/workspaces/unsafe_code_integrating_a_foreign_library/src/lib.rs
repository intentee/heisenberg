pub struct Absolute {
    pub input: i32,
}

unsafe impl Send for Absolute {}

impl Absolute {
    #[must_use]
    pub fn value(&self) -> i32 {
        unsafe { foreign_sys::abs(self.input) }
    }
}

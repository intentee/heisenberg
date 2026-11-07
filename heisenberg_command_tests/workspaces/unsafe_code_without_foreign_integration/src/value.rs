use crate::trusted::Trusted;

static OFFSET: u8 = 1;

pub struct Value {
    pub byte: u8,
}

unsafe impl Trusted for Value {}

impl Value {
    #[must_use]
    pub const fn keyword() -> &'static str {
        stringify!(unsafe)
    }

    #[must_use]
    pub fn read(&self) -> u8 {
        unsafe { Self::unchecked() }.wrapping_add(self.byte).wrapping_add(OFFSET)
    }

    #[must_use]
    pub const unsafe fn unchecked() -> u8 {
        0
    }
}

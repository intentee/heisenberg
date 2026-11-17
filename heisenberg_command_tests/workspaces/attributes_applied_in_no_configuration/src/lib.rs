pub struct Conditional;

impl Conditional {
    #[cfg_attr(rustfmt, rustfmt::skip)]
    pub const fn hidden_from_rustfmt() {}

    #[cfg_attr(target_pointer_width = "64", cfg_attr(clippy, must_use))]
    pub const fn nested() -> u8 {
        1
    }
}

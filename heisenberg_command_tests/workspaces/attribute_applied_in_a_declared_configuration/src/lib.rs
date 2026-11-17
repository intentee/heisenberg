pub struct Conditional;

impl Conditional {
    #[cfg_attr(target_pointer_width = "64", must_use)]
    pub const fn applied() -> u8 {
        1
    }
}

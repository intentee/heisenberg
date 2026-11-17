pub struct Configured;

impl Configured {
    #[cfg_attr(not(rustfmt), must_use)]
    pub const fn attributed() -> u8 {
        1
    }

    #[cfg(any(not(target_pointer_width = "64"), not(clippy)))]
    pub const fn hidden_from_clippy() {}

    #[cfg(not(target_os = "none"))]
    pub const fn hosted() {}

    pub fn statement() {
        #[cfg(not(miri))]
        println!("negated");
    }
}

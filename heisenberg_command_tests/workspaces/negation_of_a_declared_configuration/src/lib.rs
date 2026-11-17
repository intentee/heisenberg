pub struct Configured;

impl Configured {
    #[cfg(any(not(target_pointer_width = "64"), unix, windows))]
    pub const fn on_every_platform() {}
}

pub struct Never;

impl Never {
    #[cfg(any())]
    pub fn unreachable() {}
}

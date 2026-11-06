pub struct First;

impl First {
    #[cfg(all(feature = "fast", fast_seen))]
    pub fn second() -> crate::second::Second {
        crate::second::Second
    }
}

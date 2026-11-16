pub struct First;

impl First {
    #[cfg(three_jobs)]
    pub fn second() -> crate::second::Second {
        crate::second::Second
    }
}

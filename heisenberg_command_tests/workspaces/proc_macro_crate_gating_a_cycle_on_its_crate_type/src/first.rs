pub struct First;

impl First {
    #[cfg(proc_macro)]
    pub fn second() -> crate::second::Second {
        crate::second::Second
    }
}

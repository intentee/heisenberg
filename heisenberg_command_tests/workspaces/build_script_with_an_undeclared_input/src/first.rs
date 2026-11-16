pub struct First;

impl First {
    #[cfg(undeclared_input)]
    pub fn second() -> crate::second::Second {
        crate::second::Second
    }
}

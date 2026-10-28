pub struct First;

impl First {
    #[attribute::mark]
    pub fn second() -> crate::second::Second {
        crate::second::Second
    }
}

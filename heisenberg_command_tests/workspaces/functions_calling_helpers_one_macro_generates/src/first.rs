helper!();

pub struct First;

impl First {
    #[must_use]
    pub fn helped() -> u8 {
        helper()
    }

    #[must_use]
    pub const fn valued() -> u8 {
        Helper::value()
    }
}

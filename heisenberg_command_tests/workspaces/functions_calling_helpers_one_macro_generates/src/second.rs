helper!();

pub struct Second;

impl Second {
    #[must_use]
    pub fn helped() -> u8 {
        helper()
    }

    #[must_use]
    pub const fn valued() -> u8 {
        Helper::value()
    }
}

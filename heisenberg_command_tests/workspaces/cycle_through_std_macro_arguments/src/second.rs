pub struct Second;

impl Second {
    pub fn value() -> u8 {
        crate::first::First::limit()
    }
}

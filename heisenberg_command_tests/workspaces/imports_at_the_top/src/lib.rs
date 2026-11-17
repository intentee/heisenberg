use std::collections::BTreeSet;

pub struct Counted;

impl Counted {
    #[must_use]
    pub fn count() -> usize {
        BTreeSet::<u8>::new().len()
    }
}

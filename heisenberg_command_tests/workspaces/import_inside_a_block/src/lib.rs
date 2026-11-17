pub struct Counted;

impl Counted {
    #[must_use]
    pub fn count() -> usize {
        use std::collections::BTreeSet;

        BTreeSet::<u8>::new().len()
    }
}

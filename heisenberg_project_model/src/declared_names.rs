use std::collections::BTreeSet;

pub struct DeclaredNames<'names> {
    pub distinct: BTreeSet<&'names str>,
    pub repeated: BTreeSet<&'names str>,
}

impl<'names> DeclaredNames<'names> {
    #[must_use]
    pub fn new(names: &'names [String]) -> Self {
        let mut distinct = BTreeSet::new();
        let mut repeated = BTreeSet::new();

        for name in names {
            if !distinct.insert(name.as_str()) {
                repeated.insert(name.as_str());
            }
        }

        Self { distinct, repeated }
    }
}

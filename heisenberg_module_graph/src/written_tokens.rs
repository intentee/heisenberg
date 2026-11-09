use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct WrittenTokens {
    starts: BTreeMap<String, BTreeSet<u32>>,
}

impl WrittenTokens {
    #[must_use]
    pub fn contains(&self, file: &str, start: u32) -> bool {
        self.starts
            .get(file)
            .is_some_and(|starts| starts.contains(&start))
    }

    pub fn insert(&mut self, file: String, start: u32) {
        self.starts.entry(file).or_default().insert(start);
    }

    pub fn merge(&mut self, Self { starts }: Self) {
        for (file, file_starts) in starts {
            self.starts.entry(file).or_default().extend(file_starts);
        }
    }
}

use crate::misordered_entry::MisorderedEntry;
use crate::ordered_entry::OrderedEntry;

pub struct OrderedEntries<TKey> {
    pub entries: Vec<OrderedEntry<TKey>>,
}

impl<TKey: Ord> OrderedEntries<TKey> {
    pub fn misordered(&self) -> Vec<MisorderedEntry<'_, TKey>> {
        let mut earlier: Vec<&OrderedEntry<TKey>> = Vec::new();
        let mut misordered: Vec<MisorderedEntry<'_, TKey>> = Vec::new();

        for entry in &self.entries {
            misordered.extend(
                earlier
                    .iter()
                    .copied()
                    .find(|OrderedEntry { key, .. }| *key > entry.key)
                    .map(|preceding| MisorderedEntry { entry, preceding }),
            );
            earlier.push(entry);
        }

        misordered
    }
}

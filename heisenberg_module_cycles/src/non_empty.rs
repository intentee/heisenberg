use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::iter::once;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NonEmpty<TItem> {
    pub first: TItem,
    pub others: Vec<TItem>,
}

impl<TItem> NonEmpty<TItem> {
    pub fn iter(&self) -> impl Iterator<Item = &TItem> {
        once(&self.first).chain(&self.others)
    }

    pub fn push_into<TKey: Ord>(groups: &mut BTreeMap<TKey, Self>, key: TKey, item: TItem) {
        match groups.entry(key) {
            Entry::Occupied(mut group) => group.get_mut().others.push(item),
            Entry::Vacant(group) => {
                group.insert(Self {
                    first: item,
                    others: Vec::new(),
                });
            }
        }
    }
}

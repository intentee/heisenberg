use crate::ordered_entry::OrderedEntry;

pub struct MisorderedEntry<'entries, TKey> {
    pub entry: &'entries OrderedEntry<TKey>,
    pub preceding: &'entries OrderedEntry<TKey>,
}

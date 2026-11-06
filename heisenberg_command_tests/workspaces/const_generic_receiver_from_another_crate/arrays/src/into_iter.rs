use crate::sized::Sized;

pub struct IntoIter<TStorage: Sized> {
    pub storage: TStorage,
}

impl<TStorage: Sized> Iterator for IntoIter<TStorage> {
    type Item = TStorage::Item;

    fn next(&mut self) -> Option<TStorage::Item> {
        None
    }
}

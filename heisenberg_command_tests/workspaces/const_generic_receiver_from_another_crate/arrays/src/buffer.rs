use crate::into_iter::IntoIter;
use crate::sized::Sized;

pub struct Buffer<TStorage: Sized> {
    pub storage: TStorage,
}

impl<TStorage: Sized> IntoIterator for Buffer<TStorage> {
    type Item = TStorage::Item;
    type IntoIter = IntoIter<TStorage>;

    fn into_iter(self) -> IntoIter<TStorage> {
        IntoIter {
            storage: self.storage,
        }
    }
}

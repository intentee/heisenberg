use crate::into_iter::IntoIter;
use crate::sized::Sized;

pub struct Buffer<A: Sized> {
    pub storage: A,
}

impl<A: Sized> IntoIterator for Buffer<A> {
    type Item = A::Item;
    type IntoIter = IntoIter<A>;

    fn into_iter(self) -> IntoIter<A> {
        IntoIter {
            storage: self.storage,
        }
    }
}

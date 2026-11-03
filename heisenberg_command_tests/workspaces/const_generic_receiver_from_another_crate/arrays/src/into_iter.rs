use crate::sized::Sized;

pub struct IntoIter<A: Sized> {
    pub storage: A,
}

impl<A: Sized> Iterator for IntoIter<A> {
    type Item = A::Item;

    fn next(&mut self) -> Option<A::Item> {
        None
    }
}

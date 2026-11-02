pub trait Counting {
    fn counted(self) -> usize;
}

impl<TIterator: Iterator> Counting for TIterator {
    fn counted(self) -> usize {
        self.count()
    }
}

use crate::counting::Counting;

impl<TIterator: Iterator> Counting for TIterator {
    fn counted(self) -> usize {
        self.count()
    }
}

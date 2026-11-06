pub trait Sized {
    type Item;
}

impl<TElement, const LENGTH: usize> Sized for [TElement; LENGTH] {
    type Item = TElement;
}

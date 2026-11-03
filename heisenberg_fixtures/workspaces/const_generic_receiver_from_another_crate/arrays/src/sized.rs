pub trait Sized {
    type Item;
}

impl<T, const N: usize> Sized for [T; N] {
    type Item = T;
}

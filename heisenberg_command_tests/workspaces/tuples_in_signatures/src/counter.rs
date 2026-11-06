pub struct Counter;

impl Iterator for Counter {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub struct Counter;

impl Counter {
    pub fn reset(&mut self) -> Result<(), u8> {
        let pair: (u8, u8) = (1, 2);
        let matches = |entry: (u8, u8)| entry == pair;
        let pairs = vec![pair].into_iter().collect::<Vec<(u8, u8)>>();

        if pairs.into_iter().any(matches) {
            Ok(())
        } else {
            Err(0)
        }
    }
}

impl Iterator for Counter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

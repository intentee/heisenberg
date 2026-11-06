pub struct Labels<'labels, TLabel> {
    pub labels: &'labels [TLabel],
}

impl Labels<'_, &'static str> {
    pub fn count(&self, _: u8) -> usize {
        self.labels.len()
    }
}

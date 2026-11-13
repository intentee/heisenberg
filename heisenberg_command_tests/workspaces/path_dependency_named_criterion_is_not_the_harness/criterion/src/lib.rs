pub struct Criterion;

impl Criterion {
    #[must_use]
    pub const fn sample_size(self, _size: usize) -> Self {
        self
    }

    pub fn bench_function(&mut self, _name: &str, _routine: impl FnMut()) {}
}

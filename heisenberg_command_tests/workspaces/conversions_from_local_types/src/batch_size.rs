pub struct BatchSize(pub u32);

impl From<BatchSize> for u32 {
    fn from(BatchSize(size): BatchSize) -> Self {
        size
    }
}

use crate::sequence_length::SequenceLength;

impl From<SequenceLength> for u64 {
    fn from(length: SequenceLength) -> Self {
        length.0
    }
}

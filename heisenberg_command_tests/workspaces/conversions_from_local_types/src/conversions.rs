use crate::sequence_length::SequenceLength;

impl From<SequenceLength> for u64 {
    fn from(SequenceLength(length): SequenceLength) -> Self {
        length
    }
}

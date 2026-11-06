use std::ops::Range;

pub fn width(range: Range<u8>, ranges: &[Range<u8>]) -> u8 {
    ranges
        .iter()
        .map(|other| other.end)
        .fold(range.end - range.start, u8::max)
}

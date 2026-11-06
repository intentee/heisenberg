use std::ops::Range;

pub fn length(
    range: Range<u8>,
    boxed: Box<Range<u8>>,
    ended: Range<u8>,
    _unused: Range<u8>,
) -> usize {
    usize::from(range.start)
        + range.len()
        + usize::from(boxed.end)
        + usize::from(macros::end_of!(ended))
}

use crate::zero_based_position::ZeroBasedPosition;

#[derive(Clone, Copy)]
pub struct ZeroBasedSpan {
    pub start: ZeroBasedPosition,
    pub end: ZeroBasedPosition,
}

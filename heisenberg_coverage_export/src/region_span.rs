use crate::region_position::RegionPosition;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct RegionSpan {
    pub start: RegionPosition,
    pub end: RegionPosition,
}

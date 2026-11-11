use crate::constants::FIRST_REGION_POSITION;
use crate::zero_based_position::ZeroBasedPosition;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RegionPosition {
    pub line: u32,
    pub column: u32,
}

impl RegionPosition {
    #[must_use]
    pub(crate) fn zero_based(self) -> Option<ZeroBasedPosition> {
        self.line
            .checked_sub(FIRST_REGION_POSITION)
            .and_then(|line| {
                self.column
                    .checked_sub(FIRST_REGION_POSITION)
                    .map(|column| ZeroBasedPosition { line, column })
            })
    }
}

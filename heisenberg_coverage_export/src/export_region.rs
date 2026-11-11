use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::region_position::RegionPosition;
use crate::region_span::RegionSpan;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExportRegion {
    pub line_start: u32,
    pub column_start: u32,
    pub line_end: u32,
    pub column_end: u32,
    pub execution_count: u64,
    pub file_id: usize,
    pub expanded_file_id: IgnoredAny,
    pub kind: u64,
}

impl ExportRegion {
    pub const fn span(&self) -> RegionSpan {
        RegionSpan {
            end: RegionPosition {
                column: self.column_end,
                line: self.line_end,
            },
            start: RegionPosition {
                column: self.column_start,
                line: self.line_start,
            },
        }
    }
}

use crate::region_span::RegionSpan;
use crate::zero_based_span::ZeroBasedSpan;

pub struct ExportedRegion {
    pub executed: bool,
    pub exported: RegionSpan,
    pub span: ZeroBasedSpan,
}

use heisenberg_module_graph::source_location::SourceLocation;

use crate::measured_region::MeasuredRegion;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct MeasuredFunction {
    pub location: SourceLocation,
    pub regions: Vec<MeasuredRegion>,
}

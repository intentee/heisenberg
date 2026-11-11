use heisenberg_module_graph::source_location::SourceLocation;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct MeasuredRegion {
    pub executed: bool,
    pub location: SourceLocation,
}

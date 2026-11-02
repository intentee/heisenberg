use heisenberg_module_graph::source_location::SourceLocation;

use crate::cycle_witness::CycleWitness;
use crate::non_empty::NonEmpty;

#[must_use]
pub fn cycle_site(witnesses: &NonEmpty<CycleWitness>) -> &SourceLocation {
    witnesses
        .iter()
        .flat_map(|witness| witness.dependencies.iter())
        .map(|dependency| &dependency.location)
        .fold(&witnesses.first.dependencies.first.location, Ord::min)
}

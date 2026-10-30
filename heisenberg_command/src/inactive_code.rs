use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use heisenberg_module_graph::inactive_region::InactiveRegion;

use crate::target_analysis::TargetAnalysis;

pub struct InactiveCode<'code> {
    pub analyses: &'code [TargetAnalysis],
}

impl InactiveCode<'_> {
    #[must_use]
    pub fn regions(&self) -> Vec<InactiveRegion> {
        let mut inactive: BTreeMap<&str, Vec<InactiveRegion>> = BTreeMap::new();

        for graph in self.analyses.iter().flat_map(|analysis| &analysis.graphs) {
            for file in &graph.walked_files {
                let regions: Vec<InactiveRegion> = graph
                    .inactive_regions
                    .iter()
                    .filter(|region| region.start.file == *file)
                    .cloned()
                    .collect();

                match inactive.entry(file) {
                    Entry::Vacant(vacant) => {
                        vacant.insert(regions);
                    }
                    Entry::Occupied(mut occupied) => {
                        let narrowed = Self::intersection(occupied.get(), &regions);

                        *occupied.get_mut() = narrowed;
                    }
                }
            }
        }

        inactive.into_values().flatten().collect()
    }

    fn intersection(left: &[InactiveRegion], right: &[InactiveRegion]) -> Vec<InactiveRegion> {
        let mut intersection = Vec::new();
        let mut left_index = 0;
        let mut right_index = 0;

        while left_index < left.len() && right_index < right.len() {
            let left_region = &left[left_index];
            let right_region = &right[right_index];
            let start = (&left_region.start).max(&right_region.start);
            let end = (&left_region.end).min(&right_region.end);

            if start < end {
                intersection.push(InactiveRegion {
                    end: end.clone(),
                    start: start.clone(),
                });
            }

            if left_region.end < right_region.end {
                left_index += 1;
            } else {
                right_index += 1;
            }
        }

        intersection
    }
}

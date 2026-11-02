use std::cmp;
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
            for file in &graph.walked_code.files {
                let regions: Vec<InactiveRegion> = graph
                    .walked_code
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
        let mut left = left.iter().peekable();
        let mut right = right.iter().peekable();

        while let Some(&left_region) = left.peek()
            && let Some(&right_region) = right.peek()
        {
            let start = (&left_region.start).max(&right_region.start);
            let end = (&left_region.end).min(&right_region.end);

            if start < end {
                intersection.push(InactiveRegion {
                    end: end.clone(),
                    focus: cmp::max_by(left_region, right_region, |first, second| {
                        first
                            .start
                            .cmp(&second.start)
                            .then_with(|| second.end.cmp(&first.end))
                    })
                    .focus
                    .clone(),
                    start: start.clone(),
                });
            }

            if left_region.end < right_region.end {
                left.next();
            } else {
                right.next();
            }
        }

        intersection
    }
}

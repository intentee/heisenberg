use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::inactive_focus::InactiveFocus;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::walked_code::WalkedCode;

use crate::target_analysis::TargetAnalysis;

pub struct InactiveCode<'code> {
    pub analyses: &'code [TargetAnalysis],
}

impl InactiveCode<'_> {
    #[must_use]
    pub fn foci(&self) -> BTreeMap<&SourceLocation, BTreeSet<&SourceLocation>> {
        let walked: Vec<&WalkedCode> = self
            .analyses
            .iter()
            .flat_map(|TargetAnalysis { graphs, .. }| graphs)
            .map(|CrateModuleGraph { walked_code, .. }| walked_code)
            .collect();
        let mut foci: BTreeMap<&SourceLocation, BTreeSet<&SourceLocation>> = BTreeMap::new();

        for InactiveFocus {
            location,
            negations,
            token_starts,
        } in walked
            .iter()
            .flat_map(|WalkedCode { inactive_foci, .. }| inactive_foci)
        {
            let is_dead_everywhere = walked
                .iter()
                .filter(|WalkedCode { files, .. }| files.contains(&location.file))
                .all(
                    |WalkedCode {
                         active_tokens,
                         inactive_tokens,
                         ..
                     }| {
                        token_starts.iter().all(|start| {
                            inactive_tokens.contains(&location.file, *start)
                                && !active_tokens.contains(&location.file, *start)
                        })
                    },
                );

            if is_dead_everywhere {
                foci.entry(location).or_default().extend(negations);
            }
        }

        foci
    }
}

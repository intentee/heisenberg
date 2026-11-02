use std::collections::BTreeSet;

use heisenberg_module_graph::inactive_focus::InactiveFocus;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::walked_code::WalkedCode;

use crate::target_analysis::TargetAnalysis;

pub struct InactiveCode<'code> {
    pub analyses: &'code [TargetAnalysis],
}

impl InactiveCode<'_> {
    #[must_use]
    pub fn foci(&self) -> BTreeSet<&SourceLocation> {
        let walked: Vec<&WalkedCode> = self
            .analyses
            .iter()
            .flat_map(|analysis| &analysis.graphs)
            .map(|graph| &graph.walked_code)
            .collect();

        walked
            .iter()
            .flat_map(|code| &code.inactive_foci)
            .filter(
                |InactiveFocus {
                     location,
                     token_starts,
                 }| {
                    walked
                        .iter()
                        .filter(|code| code.files.contains(&location.file))
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
                        )
                },
            )
            .map(|InactiveFocus { location, .. }| location)
            .collect()
    }
}

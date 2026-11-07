use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::inactive_focus::InactiveFocus;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::walked_code::WalkedCode;

use crate::configuration_analysis::ConfigurationAnalysis;

pub struct InactiveCode<'code> {
    pub analyses: &'code [ConfigurationAnalysis],
}

impl InactiveCode<'_> {
    #[must_use]
    pub fn foci(&self) -> BTreeMap<&SourceLocation, BTreeSet<&SourceLocation>> {
        let walked: Vec<&WalkedCode> = self
            .analyses
            .iter()
            .flat_map(|ConfigurationAnalysis { graphs, .. }| graphs)
            .map(|CrateModuleGraph { walked_code, .. }| walked_code)
            .collect();
        let mut walked_files: BTreeMap<&str, Vec<&WalkedCode>> = BTreeMap::new();
        let mut foci: BTreeMap<&SourceLocation, BTreeSet<&SourceLocation>> = BTreeMap::new();

        for code in &walked {
            for file in &code.files {
                walked_files.entry(file).or_default().push(code);
            }
        }

        for InactiveFocus {
            location,
            negations,
            token_starts,
        } in walked
            .iter()
            .flat_map(|WalkedCode { inactive_foci, .. }| inactive_foci)
        {
            let is_dead_everywhere = walked_files
                .get(location.file.as_str())
                .into_iter()
                .flatten()
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

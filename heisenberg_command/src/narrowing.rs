use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_project_model::supported_target::SupportedTarget;

use crate::analysis_context::AnalysisContext;
use crate::target_list::TargetList;

fn targets_by_crate(
    contexts: &BTreeSet<AnalysisContext>,
) -> BTreeMap<&CrateTarget, BTreeSet<SupportedTarget>> {
    let mut targets_by_crate: BTreeMap<&CrateTarget, BTreeSet<SupportedTarget>> = BTreeMap::new();

    for AnalysisContext {
        crate_target,
        target,
    } in contexts
    {
        targets_by_crate
            .entry(crate_target)
            .or_default()
            .insert(*target);
    }

    targets_by_crate
}

pub struct Narrowing<'contexts> {
    pub base: &'contexts BTreeSet<AnalysisContext>,
    pub reported: &'contexts BTreeSet<AnalysisContext>,
}

impl Display for Narrowing<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        if self.reported == self.base {
            Ok(())
        } else {
            write!(
                formatter,
                " (only in {})",
                targets_by_crate(self.reported)
                    .into_iter()
                    .map(|(crate_target, targets)| {
                        let base_targets: BTreeSet<SupportedTarget> = self
                            .base
                            .iter()
                            .filter(|context| context.crate_target == *crate_target)
                            .map(|context| context.target)
                            .collect();

                        if targets == base_targets {
                            crate_target.to_string()
                        } else {
                            format!("{crate_target} on {}", TargetList { targets: &targets })
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("; ")
            )
        }
    }
}

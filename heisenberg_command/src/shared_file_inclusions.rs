use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::module_path::ModulePath;
use heisenberg_project_model::supported_target::SupportedTarget;

use crate::module_inclusion::ModuleInclusion;
use crate::target_list::TargetList;
use crate::target_narrowing::TargetNarrowing;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SharedFileInclusions {
    pub inclusions: BTreeMap<ModuleInclusion, BTreeMap<SupportedTarget, ModuleFileAnalysis>>,
}

impl SharedFileInclusions {
    #[must_use]
    pub fn lines(&self) -> Vec<String> {
        let shared_targets = self.targets();

        self.inclusions
            .iter()
            .map(
                |(
                    ModuleInclusion {
                        crate_target,
                        module,
                    },
                    analyses,
                )| {
                    let inclusion_targets: BTreeSet<SupportedTarget> =
                        analyses.keys().copied().collect();
                    let mut skipped: BTreeMap<&ModulePath, BTreeSet<SupportedTarget>> =
                        BTreeMap::new();

                    for (target, analysis) in analyses {
                        if let ModuleFileAnalysis::AlreadyAnalysedAs(first) = analysis {
                            skipped.entry(first).or_default().insert(*target);
                        }
                    }

                    format!(
                        "  {module} in {crate_target}{}{}",
                        TargetNarrowing {
                            all: &shared_targets,
                            targets: &inclusion_targets,
                        },
                        skipped
                            .iter()
                            .map(|(first, targets)| {
                                format!(
                                    ", not analysed{}: the crate already analyses the file as {first}",
                                    if *targets == inclusion_targets {
                                        String::new()
                                    } else {
                                        format!(" on {}", TargetList { targets })
                                    }
                                )
                            })
                            .collect::<Vec<String>>()
                            .concat()
                    )
                },
            )
            .collect()
    }

    #[must_use]
    pub fn targets(&self) -> BTreeSet<SupportedTarget> {
        self.inclusions
            .values()
            .flat_map(BTreeMap::keys)
            .copied()
            .collect()
    }
}

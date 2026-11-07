use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::module_path::ModulePath;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::configuration_list::ConfigurationList;

use crate::configuration_narrowing::ConfigurationNarrowing;
use crate::module_inclusion::ModuleInclusion;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SharedFileInclusions {
    pub inclusions: BTreeMap<ModuleInclusion, BTreeMap<Configuration, ModuleFileAnalysis>>,
    pub submodule_files: BTreeSet<String>,
}

impl SharedFileInclusions {
    #[must_use]
    pub fn configurations(&self) -> BTreeSet<Configuration> {
        self.inclusions
            .values()
            .flat_map(BTreeMap::keys)
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn lines(&self) -> Vec<String> {
        let shared_configurations = self.configurations();

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
                    let inclusion_configurations: BTreeSet<Configuration> =
                        analyses.keys().cloned().collect();
                    let mut skipped: BTreeMap<&ModulePath, BTreeSet<Configuration>> =
                        BTreeMap::new();

                    for (configuration, analysis) in analyses {
                        if let ModuleFileAnalysis::AlreadyAnalysedAs(first) = analysis {
                            skipped
                                .entry(first)
                                .or_default()
                                .insert(configuration.clone());
                        }
                    }

                    format!(
                        "  {module} in {crate_target}{}{}",
                        ConfigurationNarrowing {
                            all: &shared_configurations,
                            configurations: &inclusion_configurations,
                        },
                        skipped
                            .iter()
                            .map(|(first, configurations)| {
                                format!(
                                    ", not analysed{}: the crate already analyses the file as {first}",
                                    if *configurations == inclusion_configurations {
                                        String::new()
                                    } else {
                                        format!(" on {}", ConfigurationList { configurations })
                                    }
                                )
                            })
                            .collect::<Vec<String>>()
                            .concat()
                    )
                },
            )
            .chain(
                (!self.submodule_files.is_empty())
                    .then(|| "  files of its submodules:".to_owned()),
            )
            .chain(self.submodule_files.iter().map(|file| format!("    {file}")))
            .collect()
    }
}

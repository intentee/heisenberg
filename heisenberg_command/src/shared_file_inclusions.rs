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
    pub submodule_files: BTreeSet<String>,
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
            .chain(
                (!self.submodule_files.is_empty())
                    .then(|| "  files of its submodules:".to_owned()),
            )
            .chain(self.submodule_files.iter().map(|file| format!("    {file}")))
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    use heisenberg_module_graph::crate_target::CrateTarget;
    use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
    use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
    use heisenberg_module_graph::module_path::ModulePath;
    use heisenberg_module_graph::test_harness::TestHarness;
    use heisenberg_project_model::supported_target::SupportedTarget;

    use super::SharedFileInclusions;
    use crate::module_inclusion::ModuleInclusion;

    #[test]
    fn inclusion_skipped_on_some_of_its_platforms_names_them() {
        let library = CrateTarget {
            kind: CrateTargetKind::Library,
            name: "shared".to_owned(),
            package_name: "shared".to_owned(),
            target_description: "lib".to_owned(),
            test_harness: TestHarness::Libtest,
        };
        let first = ModulePath {
            segments: vec!["first".to_owned()],
        };

        assert_eq!(
            SharedFileInclusions {
                inclusions: BTreeMap::from([
                    (
                        ModuleInclusion {
                            crate_target: library.clone(),
                            module: first.clone(),
                        },
                        BTreeMap::from([
                            (SupportedTarget::X86_64PcWindowsMsvc, ModuleFileAnalysis::Analysed),
                            (SupportedTarget::X86_64UnknownLinuxGnu, ModuleFileAnalysis::Analysed),
                        ]),
                    ),
                    (
                        ModuleInclusion {
                            crate_target: library,
                            module: ModulePath {
                                segments: vec!["second".to_owned()],
                            },
                        },
                        BTreeMap::from([
                            (
                                SupportedTarget::X86_64PcWindowsMsvc,
                                ModuleFileAnalysis::AlreadyAnalysedAs(first),
                            ),
                            (SupportedTarget::X86_64UnknownLinuxGnu, ModuleFileAnalysis::Analysed),
                        ]),
                    ),
                ]),
                submodule_files: BTreeSet::new(),
            }
            .lines(),
            vec![
                "  crate::first in shared lib".to_owned(),
                "  crate::second in shared lib, not analysed on x86_64-pc-windows-msvc: the crate already analyses the file as crate::first".to_owned(),
            ]
        );
    }
}

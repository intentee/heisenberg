use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::violation::Violation;
use heisenberg_project_model::package_violation::PackageViolation;
use heisenberg_project_model::supported_target::SupportedTarget;

use crate::analysis_context::AnalysisContext;
use crate::constants::SUPPORTED_RUST_VERSION;
use crate::cycle_shape::CycleShape;
use crate::heisenberg_exit_status::HeisenbergExitStatus;
use crate::inactive_code::InactiveCode;
use crate::issue::Issue;
use crate::module_inclusion::ModuleInclusion;
use crate::target_analysis::TargetAnalysis;

const SEPARATOR: &str = "------------------------------";

pub struct CheckReport {
    pub analyses: Vec<TargetAnalysis>,
    pub package_violations: Vec<PackageViolation>,
}

impl CheckReport {
    #[must_use]
    pub fn render(&self) -> String {
        let issues = self.issues();
        let contexts_by_file = self.contexts_by_file();
        let mut lines: Vec<String> = Vec::new();

        for issue in &issues {
            lines.extend(issue.lines(&contexts_by_file));
            lines.push(String::new());
        }

        lines.push(SEPARATOR.to_owned());
        lines.push(if issues.is_empty() {
            "No errors found!".to_owned()
        } else {
            format!("{} errors found", issues.len())
        });
        lines.push(SEPARATOR.to_owned());
        lines.extend(
            self.analyses
                .iter()
                .map(|TargetAnalysis { graphs, target, .. }| {
                    format!(
                        "Analysed on {}: {} crate targets, {} modules",
                        target.triple(),
                        graphs.len(),
                        graphs
                            .iter()
                            .map(|CrateModuleGraph { modules, .. }| modules.len())
                            .sum::<usize>()
                    )
                }),
        );
        lines.push(format!(
            "Precondition: rustc {SUPPORTED_RUST_VERSION} compiles the workspace for every supported target"
        ));

        lines.join("\n") + "\n"
    }

    #[must_use]
    pub fn status(&self) -> HeisenbergExitStatus {
        if self.issues().is_empty() {
            HeisenbergExitStatus::Compliant
        } else {
            HeisenbergExitStatus::Violations
        }
    }

    fn contexts_by_file(&self) -> BTreeMap<String, BTreeSet<AnalysisContext>> {
        let mut contexts_by_file: BTreeMap<String, BTreeSet<AnalysisContext>> = BTreeMap::new();

        for analysis in &self.analyses {
            for graph in &analysis.graphs {
                for file in &graph.walked_files {
                    contexts_by_file
                        .entry(file.clone())
                        .or_default()
                        .insert(AnalysisContext {
                            crate_target: graph.crate_target.clone(),
                            target: analysis.target,
                        });
                }
            }
        }

        contexts_by_file
    }

    fn issues(&self) -> BTreeSet<Issue> {
        let mut violations: BTreeMap<&Violation, BTreeSet<AnalysisContext>> = BTreeMap::new();
        let mut cycles: BTreeMap<CycleShape<'_>, BTreeSet<AnalysisContext>> = BTreeMap::new();
        let mut shared_files: BTreeMap<
            &str,
            BTreeMap<ModuleInclusion, BTreeMap<SupportedTarget, ModuleFileAnalysis>>,
        > = BTreeMap::new();

        for analysis in &self.analyses {
            for graph in &analysis.graphs {
                for violation in &graph.violations {
                    violations
                        .entry(violation)
                        .or_default()
                        .insert(AnalysisContext {
                            crate_target: graph.crate_target.clone(),
                            target: analysis.target,
                        });
                }

                for shared in &graph.shared_module_files {
                    shared_files
                        .entry(shared.file.as_str())
                        .or_default()
                        .entry(ModuleInclusion {
                            crate_target: graph.crate_target.clone(),
                            module: shared.module.clone(),
                        })
                        .or_default()
                        .insert(analysis.target, shared.analysis.clone());
                }
            }

            for cycle in &analysis.cycles {
                cycles
                    .entry(CycleShape {
                        container: &cycle.container,
                        witnesses: &cycle.witnesses,
                    })
                    .or_default()
                    .insert(AnalysisContext {
                        crate_target: cycle.crate_target.clone(),
                        target: analysis.target,
                    });
            }
        }

        let mut issues = BTreeSet::new();

        for (violation, reported) in violations {
            issues.insert(Issue::new_violation(violation, reported));
        }

        for (shape, reported) in cycles {
            issues.insert(Issue::new_cycle(shape, reported));
        }

        for (file, inclusions) in shared_files {
            issues.insert(Issue::new_shared_module_file(file, inclusions));
        }

        for region in &(InactiveCode {
            analyses: &self.analyses,
        })
        .regions()
        {
            issues.insert(Issue::new_inactive_code(region));
        }

        for violation in &self.package_violations {
            issues.insert(Issue::new_package(violation));
        }

        issues
    }
}

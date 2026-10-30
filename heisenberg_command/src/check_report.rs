use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_project_model::package_violation::PackageViolation;
use heisenberg_project_model::supported_target::SupportedTarget;

use crate::crate_module::CrateModule;
use crate::crate_violation::CrateViolation;
use crate::finding::Finding;
use crate::heisenberg_exit_status::HeisenbergExitStatus;
use crate::inactive_code::InactiveCode;
use crate::shared_file::SharedFile;
use crate::target_analysis::TargetAnalysis;
use crate::targeted_finding::TargetedFinding;

pub struct CheckReport {
    pub analyses: Vec<TargetAnalysis>,
    pub package_violations: Vec<PackageViolation>,
}

impl CheckReport {
    #[must_use]
    pub fn findings(&self) -> Vec<TargetedFinding> {
        let mut merged: BTreeMap<Finding, BTreeSet<SupportedTarget>> = BTreeMap::new();

        for analysis in &self.analyses {
            let mut shared_files: BTreeMap<&str, BTreeSet<CrateModule>> = BTreeMap::new();

            for graph in &analysis.graphs {
                for shared in &graph.shared_module_files {
                    shared_files
                        .entry(shared.file.as_str())
                        .or_default()
                        .insert(CrateModule {
                            module: shared.module.clone(),
                            crate_target: graph.crate_target.clone(),
                        });
                }
            }

            let cycles = analysis.cycles.iter().cloned().map(Finding::Cycle);
            let violations = analysis.graphs.iter().flat_map(|graph| {
                graph.violations.iter().map(|violation| {
                    Finding::Violation(CrateViolation {
                        crate_target: graph.crate_target.clone(),
                        violation: violation.clone(),
                    })
                })
            });
            let shared = shared_files.into_iter().map(|(file, modules)| {
                Finding::SharedFile(SharedFile {
                    file: file.to_owned(),
                    modules,
                })
            });

            for finding in cycles.chain(violations).chain(shared) {
                merged.entry(finding).or_default().insert(analysis.target);
            }
        }

        merged
            .into_iter()
            .map(|(finding, targets)| TargetedFinding { finding, targets })
            .collect()
    }

    #[must_use]
    pub fn render(&self) -> String {
        let findings = self.findings();
        let inactive_regions = InactiveCode {
            analyses: &self.analyses,
        }
        .regions();
        let mut lines = vec![
            "heisenberg check".to_owned(),
            "precondition: rustc 1.99.1 compiles the workspace for every supported target"
                .to_owned(),
        ];
        let mut cycle_count = 0;
        let mut violation_count = inactive_regions.len() + self.package_violations.len();

        lines.extend(self.analyses.iter().map(|analysis| {
            format!(
                "analysed on {}: {} crate targets, {} modules",
                analysis.target.triple(),
                analysis.graphs.len(),
                analysis
                    .graphs
                    .iter()
                    .map(|graph| graph.modules.len())
                    .sum::<usize>()
            )
        }));

        for targeted in &findings {
            let targets = targeted
                .targets
                .iter()
                .map(|target| target.triple())
                .collect::<Vec<&str>>()
                .join(", ");

            match &targeted.finding {
                Finding::Cycle(cycle) => {
                    cycle_count += 1;
                    lines.push(format!(
                        "cycle in {} among the parts of {} (targets: {targets}):",
                        cycle.crate_target, cycle.container
                    ));
                    for witness in &cycle.witnesses {
                        lines.push(format!("  {} -> {}:", witness.from, witness.to));
                        lines.extend(witness.dependencies.iter().map(|dependency| {
                            format!(
                                "    {} uses {} at {}",
                                dependency.from, dependency.to, dependency.location
                            )
                        }));
                    }
                }
                Finding::Violation(crate_violation) => {
                    violation_count += 1;
                    lines.push(format!(
                        "violation in {}: {} at {} (targets: {targets})",
                        crate_violation.crate_target,
                        crate_violation.violation.kind,
                        crate_violation.violation.location
                    ));
                }
                Finding::SharedFile(shared_file) => {
                    violation_count += 1;
                    lines.push(format!(
                        "violation: module file {} belongs to {} modules (targets: {targets}):",
                        shared_file.file,
                        shared_file.modules.len()
                    ));
                    lines.extend(shared_file.modules.iter().map(|crate_module| {
                        format!("  {} in {}", crate_module.module, crate_module.crate_target)
                    }));
                }
            }
        }

        lines.extend(inactive_regions.iter().map(|region| {
            format!(
                "violation: code is active on no supported target at {}",
                region.start
            )
        }));
        lines.extend(
            self.package_violations
                .iter()
                .map(|package_violation| format!("violation: {package_violation}")),
        );
        lines.push(format!(
            "result: {cycle_count} cycles, {violation_count} violations"
        ));

        lines.join("\n") + "\n"
    }

    #[must_use]
    pub fn status(&self) -> HeisenbergExitStatus {
        let inactive_regions = InactiveCode {
            analyses: &self.analyses,
        }
        .regions();

        if self.findings().is_empty()
            && self.package_violations.is_empty()
            && inactive_regions.is_empty()
        {
            HeisenbergExitStatus::Compliant
        } else {
            HeisenbergExitStatus::Violations
        }
    }
}

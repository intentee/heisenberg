use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_project_model::supported_target::SupportedTarget;

use crate::crate_violation::CrateViolation;
use crate::finding::Finding;
use crate::heisenberg_exit_status::HeisenbergExitStatus;
use crate::target_analysis::TargetAnalysis;
use crate::targeted_finding::TargetedFinding;

pub struct CheckReport {
    pub analyses: Vec<TargetAnalysis>,
}

impl CheckReport {
    #[must_use]
    pub fn findings(&self) -> Vec<TargetedFinding> {
        let mut merged: BTreeMap<Finding, BTreeSet<SupportedTarget>> = BTreeMap::new();

        for analysis in &self.analyses {
            let cycles = analysis.cycles.iter().cloned().map(Finding::Cycle);
            let violations = analysis.graphs.iter().flat_map(|graph| {
                graph.violations.iter().map(|violation| {
                    Finding::Violation(CrateViolation {
                        crate_target: graph.crate_target.clone(),
                        violation: violation.clone(),
                    })
                })
            });

            for finding in cycles.chain(violations) {
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
        let mut lines = vec![
            "heisenberg check".to_owned(),
            "precondition: rustc 1.99.1 compiles the workspace for every supported target"
                .to_owned(),
        ];
        let mut cycle_count = 0;
        let mut violation_count = 0;

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
                    lines.extend(cycle.witnesses.iter().map(|witness| {
                        format!(
                            "  {} -> {}: {} uses {} at {}",
                            witness.from,
                            witness.to,
                            witness.dependency.from,
                            witness.dependency.to,
                            witness.dependency.location
                        )
                    }));
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
            }
        }

        lines.push(format!(
            "result: {cycle_count} cycles, {violation_count} violations"
        ));

        lines.join("\n") + "\n"
    }

    #[must_use]
    pub fn status(&self) -> HeisenbergExitStatus {
        if self.findings().is_empty() {
            HeisenbergExitStatus::Compliant
        } else {
            HeisenbergExitStatus::Violations
        }
    }
}

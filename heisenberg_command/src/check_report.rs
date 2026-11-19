use std::collections::BTreeMap;
use std::collections::BTreeSet;

use clap::ValueEnum;

use heisenberg_module_cycles::non_empty::NonEmpty;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::folded_finding::FoldedFinding;
use heisenberg_module_graph::folded_finding_kind::FoldedFindingKind;
use heisenberg_module_graph::foreign_module::ForeignModule;
use heisenberg_module_graph::function_body::FunctionBody;
use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_module_graph::module_findings::ModuleFindings;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_module_graph::unresolved_foreign_reference::UnresolvedForeignReference;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_project_model::build_script_variables::BuildScriptVariables;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::configuration_list::ConfigurationList;
use heisenberg_project_model::configuration_reach::ConfigurationReach;
use heisenberg_project_model::confinement_breach::ConfinementBreach;
use heisenberg_project_model::constants::SUPPORTED_RUST_VERSION;
use heisenberg_project_model::package_violation::PackageViolation;
use heisenberg_project_model::run_scope::RunScope;

use crate::analysis_context::AnalysisContext;
use crate::analysis_contexts::AnalysisContexts;
use crate::compliance_summary::ComplianceSummary;
use crate::configuration_analysis::ConfigurationAnalysis;
use crate::configuration_fingerprint::ConfigurationFingerprint;
use crate::configuration_function_body::ConfigurationFunctionBody;
use crate::cycle_shape::CycleShape;
use crate::inactive_code::InactiveCode;
use crate::issue::Issue;
use crate::issue_report::IssueReport;
use crate::issue_type::IssueType;
use crate::module_inclusion::ModuleInclusion;
use crate::nextest_presence_verdict::NextestPresenceVerdict;
use crate::rendered_report::RenderedReport;
use crate::shared_file::SharedFile;
use crate::shared_file_tree::SharedFileTree;
use crate::violation_occurrences::ViolationOccurrences;
use crate::violation_report::ViolationReport;

fn shared_module_file_issues(
    shared_files: &BTreeMap<
        &str,
        BTreeMap<ModuleInclusion, BTreeMap<Configuration, ModuleFileAnalysis>>,
    >,
) -> Vec<Issue> {
    let tree = SharedFileTree::new(shared_files);
    let mut subtrees: BTreeMap<SharedFile<'_>, BTreeSet<String>> = BTreeMap::new();

    for (file, inclusions) in shared_files {
        let root = tree.root(SharedFile { file, inclusions });
        let submodule_files = subtrees.entry(root).or_default();

        if root.file != *file {
            submodule_files.insert((*file).to_owned());
        }
    }

    subtrees
        .into_iter()
        .map(|(root, submodule_files)| {
            Issue::new_shared_module_file(root.file, root.inclusions.clone(), submodule_files)
        })
        .collect()
}

fn violation_reports(
    violations: BTreeMap<&Violation, ViolationOccurrences>,
) -> Vec<ViolationReport> {
    let mut reports: Vec<ViolationReport> = Vec::new();

    for (violation, occurrences) in violations {
        match reports.last_mut() {
            Some(report) if report.reports_the_defect_of(violation) => {
                report.absorb(violation, occurrences);
            }
            _ => reports.push(ViolationReport::new(violation, occurrences)),
        }
    }

    reports
}

pub struct CheckReport {
    pub analyses: Vec<ConfigurationAnalysis>,
    pub build_script_variables: Vec<BuildScriptVariables>,
    pub nextest_presence: NextestPresenceVerdict,
    pub package_violations: Vec<PackageViolation>,
    pub scope: RunScope,
}

impl CheckReport {
    #[must_use]
    pub fn render(&self) -> RenderedReport {
        let mut footer: Vec<String> = self
            .analyses
            .iter()
            .map(
                |ConfigurationAnalysis {
                     configuration,
                     graphs,
                     ..
                 }| {
                    format!(
                        "Analysed on {}: {} crate targets, {} modules",
                        configuration.name,
                        graphs.len(),
                        graphs
                            .iter()
                            .map(|CrateModuleGraph { module_count, .. }| module_count)
                            .sum::<usize>()
                    )
                },
            )
            .collect();

        let summary = self.compliance_summary();

        if self.scope.is_partial() {
            footer.push(format!(
                "Not analysed: {}",
                ConfigurationList {
                    configurations: &self.scope.left_out_configurations,
                }
            ));
            footer.extend(summary.partial_run_lines());
            footer.push(format!(
                "Precondition: rustc {SUPPORTED_RUST_VERSION} compiles the workspace in every analysed configuration"
            ));
        } else {
            footer.push(format!(
                "Precondition: rustc {SUPPORTED_RUST_VERSION} compiles the workspace in every declared configuration"
            ));
        }

        footer.extend(summary.summary_lines());

        IssueReport {
            contexts: &self.analysis_contexts(),
            footer,
            issues: self.issues(),
        }
        .render()
    }

    fn analysis_contexts(&self) -> AnalysisContexts {
        let mut files: BTreeMap<String, BTreeSet<AnalysisContext>> = BTreeMap::new();

        for analysis in &self.analyses {
            for graph in &analysis.graphs {
                for file in &graph.findings.walked_code.files {
                    files
                        .entry(file.clone())
                        .or_default()
                        .insert(AnalysisContext {
                            configuration: analysis.configuration.clone(),
                            crate_target: graph.crate_target.clone(),
                        });
                }
            }
        }

        AnalysisContexts {
            configurations: self
                .analyses
                .iter()
                .map(|ConfigurationAnalysis { configuration, .. }| configuration.clone())
                .collect(),
            files,
        }
    }

    fn build_script_variable_issues(&self) -> Vec<Issue> {
        self.build_script_variables
            .iter()
            .cloned()
            .map(Issue::BuildScriptVariables)
            .collect()
    }

    fn compliance_summary(&self) -> ComplianceSummary {
        let rules: Vec<IssueType> = IssueType::value_variants()
            .iter()
            .copied()
            .filter(|issue_type| !issue_type.measures_coverage())
            .collect();

        if self.scope.is_partial() {
            ComplianceSummary::new_partial(
                &rules,
                match self.nextest_presence {
                    NextestPresenceVerdict::NotEvaluated => &[],
                    NextestPresenceVerdict::Compliant | NextestPresenceVerdict::Violation(_) => &[
                        IssueType::NextestConfigurationMissing,
                        IssueType::UnneededNextestConfiguration,
                    ],
                },
            )
        } else {
            ComplianceSummary::new_full(&rules)
        }
    }

    fn confinement_breach_issues(&self) -> Vec<Issue> {
        ConfinementBreach::grouped(self.analyses.iter().map(
            |ConfigurationAnalysis {
                 configuration,
                 confinement,
                 ..
             }| ConfigurationReach {
                configuration,
                reach: confinement,
            },
        ))
        .into_iter()
        .map(|breach| Issue::ConfinedPackageReached { breach })
        .collect()
    }

    fn duplicated_function_body_issues(&self) -> Vec<Issue> {
        let bodies: BTreeSet<ConfigurationFunctionBody<'_>> = self
            .analyses
            .iter()
            .flat_map(
                |ConfigurationAnalysis {
                     configuration,
                     graphs,
                     ..
                 }| {
                    graphs
                        .iter()
                        .flat_map(
                            |CrateModuleGraph {
                                 function_bodies, ..
                             }| function_bodies,
                        )
                        .map(|body| ConfigurationFunctionBody {
                            body,
                            configuration,
                        })
                },
            )
            .collect();
        let mut copies: BTreeMap<ConfigurationFingerprint<'_>, NonEmpty<&SourceLocation>> =
            BTreeMap::new();

        for ConfigurationFunctionBody {
            body:
                FunctionBody {
                    fingerprint,
                    location,
                },
            configuration,
        } in &bodies
        {
            NonEmpty::push_into(
                &mut copies,
                ConfigurationFingerprint {
                    configuration,
                    fingerprint,
                },
                location,
            );
        }

        let mut duplicates: BTreeMap<&NonEmpty<&SourceLocation>, BTreeSet<Configuration>> =
            BTreeMap::new();

        for (ConfigurationFingerprint { configuration, .. }, group) in &copies {
            if !group.others.is_empty() {
                duplicates
                    .entry(group)
                    .or_default()
                    .insert((*configuration).clone());
            }
        }

        duplicates
            .into_iter()
            .map(|(group, configurations)| {
                Issue::new_duplicated_function_body(group, configurations)
            })
            .collect()
    }

    fn inactive_code_issues<'report>(
        &'report self,
        violations: &mut BTreeMap<&'report Violation, ViolationOccurrences>,
    ) -> Vec<Issue> {
        let negated_at: BTreeMap<&SourceLocation, &'report Violation> = violations
            .keys()
            .filter(|Violation { kind, .. }| *kind == ViolationKind::NegatedConfiguration)
            .map(|violation| (&violation.location, *violation))
            .collect();
        let mut issues = Vec::new();

        for (focus, negations) in (InactiveCode {
            analyses: &self.analyses,
        })
        .foci()
        {
            let negated: Vec<&Violation> = negations
                .iter()
                .filter_map(|location| negated_at.get(location).copied())
                .collect();

            if negated.is_empty() {
                issues.push(Issue::new_inactive_code(focus));
            }

            for violation in negated {
                violations.entry(violation).and_modify(
                    |ViolationOccurrences { folded, .. }| {
                        folded.insert(FoldedFinding {
                            kind: FoldedFindingKind::InactiveCode,
                            location: focus.clone(),
                        });
                    },
                );
            }
        }

        issues
    }

    fn issues(&self) -> Vec<Issue> {
        let mut violations: BTreeMap<&Violation, ViolationOccurrences> = BTreeMap::new();
        let mut cycles: BTreeMap<CycleShape<'_>, BTreeSet<AnalysisContext>> = BTreeMap::new();
        let mut shared_files: BTreeMap<
            &str,
            BTreeMap<ModuleInclusion, BTreeMap<Configuration, ModuleFileAnalysis>>,
        > = BTreeMap::new();

        let declarations_of_negated_files: BTreeSet<&SourceLocation> = self
            .analyses
            .iter()
            .flat_map(|ConfigurationAnalysis { graphs, .. }| graphs)
            .flat_map(
                |CrateModuleGraph {
                     findings:
                         ModuleFindings {
                             configured_out_negations,
                             ..
                         },
                     ..
                 }| &configured_out_negations.declarations,
            )
            .collect();

        for analysis in &self.analyses {
            for graph in &analysis.graphs {
                let context = AnalysisContext {
                    configuration: analysis.configuration.clone(),
                    crate_target: graph.crate_target.clone(),
                };

                for (violation, folded_findings) in &graph.findings.violations {
                    let occurrences = violations.entry(violation).or_default();

                    occurrences.reported.insert(context.clone());
                    occurrences.folded.extend(folded_findings.iter().cloned());
                }

                for negation in graph
                    .findings
                    .configured_out_negations
                    .negations
                    .iter()
                    .filter(|Violation { location, .. }| {
                        !declarations_of_negated_files.contains(location)
                    })
                {
                    violations
                        .entry(negation)
                        .or_default()
                        .reported
                        .insert(context.clone());
                }

                for shared in &graph.findings.shared_module_files {
                    shared_files
                        .entry(shared.file.as_str())
                        .or_default()
                        .entry(ModuleInclusion {
                            crate_target: graph.crate_target.clone(),
                            module: shared.module.clone(),
                        })
                        .or_default()
                        .insert(analysis.configuration.clone(), shared.analysis.clone());
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
                        configuration: analysis.configuration.clone(),
                        crate_target: cycle.crate_target.clone(),
                    });
            }
        }

        let mut issues = Vec::new();

        if !self.scope.is_partial() {
            issues.extend(self.inactive_code_issues(&mut violations));

            self.report_every_configuration_findings(&mut violations);
        }

        issues.extend(self.unresolved_foreign_reference_issues(&mut violations));

        issues.extend(
            violation_reports(violations)
                .into_iter()
                .map(Issue::new_violation),
        );

        issues.extend(
            cycles
                .into_iter()
                .map(|(shape, reported)| Issue::new_cycle(shape, reported)),
        );

        issues.extend(shared_module_file_issues(&shared_files));
        issues.extend(self.build_script_variable_issues());
        issues.extend(self.confinement_breach_issues());
        issues.extend(self.duplicated_function_body_issues());

        if let NextestPresenceVerdict::Violation(violation) = &self.nextest_presence {
            issues.push(Issue::new_package(violation));
        }

        for violation in &self.package_violations {
            issues.push(Issue::new_package(violation));
        }

        issues
    }

    fn report_every_configuration_findings<'report>(
        &'report self,
        violations: &mut BTreeMap<&'report Violation, ViolationOccurrences>,
    ) {
        let refuted: BTreeSet<&Violation> = self
            .analyses
            .iter()
            .flat_map(|ConfigurationAnalysis { graphs, .. }| graphs)
            .flat_map(|CrateModuleGraph { findings, .. }| &findings.every_configuration.refuted)
            .collect();

        for ConfigurationAnalysis {
            configuration,
            graphs,
            ..
        } in &self.analyses
        {
            for CrateModuleGraph {
                crate_target,
                findings,
                ..
            } in graphs
            {
                for finding in findings
                    .every_configuration
                    .held
                    .iter()
                    .filter(|finding| !refuted.contains(finding))
                {
                    violations
                        .entry(finding)
                        .or_default()
                        .reported
                        .insert(AnalysisContext {
                            configuration: configuration.clone(),
                            crate_target: crate_target.clone(),
                        });
                }
            }
        }
    }

    fn unresolved_foreign_reference_issues<'report>(
        &'report self,
        violations: &mut BTreeMap<&'report Violation, ViolationOccurrences>,
    ) -> Vec<Issue> {
        let references: BTreeSet<&UnresolvedForeignReference> = self
            .analyses
            .iter()
            .flat_map(|ConfigurationAnalysis { graphs, .. }| graphs)
            .flat_map(
                |CrateModuleGraph {
                     findings:
                         ModuleFindings {
                             unresolved_foreign_references,
                             ..
                         },
                     ..
                 }| unresolved_foreign_references,
            )
            .collect();

        let mut sites: BTreeMap<&ForeignModule, NonEmpty<&SourceLocation>> = BTreeMap::new();
        let mut failed_in: BTreeMap<&str, Vec<&'report Violation>> = BTreeMap::new();

        for UnresolvedForeignReference { location, module } in references {
            NonEmpty::push_into(&mut sites, module, location);
        }

        for violation in violations
            .keys()
            .filter(|Violation { kind, .. }| *kind == ViolationKind::FailedMacroExpansion)
        {
            failed_in
                .entry(violation.location.file.as_str())
                .or_default()
                .push(violation);
        }

        let mut issues = Vec::new();

        for (module, group) in sites {
            let failed = failed_in.get(module.file.as_str());

            if failed.is_none() {
                issues.push(Issue::new_unresolved_foreign_references(
                    module,
                    &group,
                    self.analyses
                        .iter()
                        .filter(|ConfigurationAnalysis { graphs, .. }| {
                            graphs
                                .iter()
                                .flat_map(
                                    |CrateModuleGraph {
                                         findings:
                                             ModuleFindings {
                                                 unresolved_foreign_references,
                                                 ..
                                             },
                                         ..
                                     }| {
                                        unresolved_foreign_references
                                    },
                                )
                                .any(
                                    |UnresolvedForeignReference {
                                         module: referenced, ..
                                     }| referenced == module,
                                )
                        })
                        .map(|ConfigurationAnalysis { configuration, .. }| configuration.clone())
                        .collect(),
                ));
            }

            for violation in failed.into_iter().flatten() {
                violations.entry(violation).and_modify(
                    |ViolationOccurrences { folded, .. }| {
                        folded.insert(FoldedFinding {
                            kind: FoldedFindingKind::UnresolvedForeignReferences {
                                count: group.iter().count(),
                                module: module.name.clone(),
                            },
                            location: group.first.clone(),
                        });
                    },
                );
            }
        }

        issues
    }
}

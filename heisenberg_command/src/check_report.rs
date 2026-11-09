use std::collections::BTreeMap;
use std::collections::BTreeSet;

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
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::configuration_list::ConfigurationList;
use heisenberg_project_model::package_violation::PackageViolation;
use heisenberg_project_model::run_scope::RunScope;

use crate::analysis_context::AnalysisContext;
use crate::configuration_analysis::ConfigurationAnalysis;
use crate::configuration_fingerprint::ConfigurationFingerprint;
use crate::configuration_function_body::ConfigurationFunctionBody;
use crate::constants::CODE_ACTIVE_IN_NO_CONFIGURATION;
use crate::constants::SUPPORTED_RUST_VERSION;
use crate::constants::TESTS_ONLY_PACKAGE_WITHOUT_TEST_LIBRARIES;
use crate::cycle_shape::CycleShape;
use crate::heisenberg_exit_status::HeisenbergExitStatus;
use crate::inactive_code::InactiveCode;
use crate::issue::Issue;
use crate::issue_section::IssueSection;
use crate::issue_type::IssueType;
use crate::located_issue::LocatedIssue;
use crate::module_inclusion::ModuleInclusion;
use crate::rendered_report::RenderedReport;
use crate::shared_file::SharedFile;
use crate::shared_file_tree::SharedFileTree;
use crate::violation_occurrences::ViolationOccurrences;

const SEPARATOR: &str = "------------------------------";

fn sorted_issues(issues: Vec<Issue>) -> Vec<Issue> {
    let mut located: Vec<LocatedIssue> = issues
        .into_iter()
        .map(|issue| LocatedIssue {
            location: issue.location(),
            issue,
        })
        .collect();

    located.sort();
    located.dedup();

    located
        .into_iter()
        .map(|LocatedIssue { issue, .. }| issue)
        .collect()
}

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

pub struct CheckReport {
    pub analyses: Vec<ConfigurationAnalysis>,
    pub package_violations: Vec<PackageViolation>,
    pub scope: RunScope,
}

impl CheckReport {
    #[must_use]
    pub fn render(&self) -> RenderedReport {
        let issues = self.issues();
        let contexts_by_file = self.contexts_by_file();
        let mut lines: Vec<String> = Vec::new();
        let mut sections: BTreeMap<IssueType, NonEmpty<&Issue>> = BTreeMap::new();

        for issue in &issues {
            NonEmpty::push_into(&mut sections, issue.issue_type(), issue);
        }

        for (issue_type, issues) in &sections {
            lines.extend(
                IssueSection {
                    issue_type: *issue_type,
                    issues,
                }
                .lines(&contexts_by_file),
            );
            lines.push(String::new());
        }

        lines.push(SEPARATOR.to_owned());
        lines.push(if issues.is_empty() {
            "No errors found!".to_owned()
        } else {
            format!("{} errors found", issues.len())
        });
        lines.push(SEPARATOR.to_owned());
        lines.extend(self.analyses.iter().map(
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
                        .map(|CrateModuleGraph { modules, .. }| modules.len())
                        .sum::<usize>()
                )
            },
        ));
        if self.scope.is_partial() {
            lines.extend(self.partial_run_lines());
        } else {
            lines.push(format!(
                "Precondition: rustc {SUPPORTED_RUST_VERSION} compiles the workspace in every declared configuration"
            ));
        }

        RenderedReport {
            status: if issues.is_empty() {
                HeisenbergExitStatus::Compliant
            } else {
                HeisenbergExitStatus::Violations
            },
            text: lines.join("\n") + "\n",
        }
    }

    fn contexts_by_file(&self) -> BTreeMap<String, BTreeSet<AnalysisContext>> {
        let mut contexts_by_file: BTreeMap<String, BTreeSet<AnalysisContext>> = BTreeMap::new();

        for analysis in &self.analyses {
            for graph in &analysis.graphs {
                for file in &graph.findings.walked_code.files {
                    contexts_by_file
                        .entry(file.clone())
                        .or_default()
                        .insert(AnalysisContext {
                            configuration: analysis.configuration.clone(),
                            crate_target: graph.crate_target.clone(),
                        });
                }
            }
        }

        contexts_by_file
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
        }

        issues.extend(self.unresolved_foreign_reference_issues(&mut violations));

        for (violation, occurrences) in violations {
            issues.push(Issue::new_violation(violation, occurrences));
        }

        issues.extend(
            cycles
                .into_iter()
                .map(|(shape, reported)| Issue::new_cycle(shape, reported)),
        );

        issues.extend(shared_module_file_issues(&shared_files));
        issues.extend(self.duplicated_function_body_issues());

        for violation in &self.package_violations {
            issues.push(Issue::new_package(violation));
        }

        sorted_issues(issues)
    }

    fn partial_run_lines(&self) -> Vec<String> {
        let RunScope {
            left_out_configurations,
        } = &self.scope;

        [
            format!(
                "Not analysed: {}",
                ConfigurationList {
                    configurations: left_out_configurations,
                }
            ),
            format!(
                "Not evaluated in a partial run: {}, the empty-library check of {}",
                CODE_ACTIVE_IN_NO_CONFIGURATION.name, TESTS_ONLY_PACKAGE_WITHOUT_TEST_LIBRARIES.name
            ),
            format!(
                "Precondition: rustc {SUPPORTED_RUST_VERSION} compiles the workspace in every analysed configuration"
            ),
        ]
        .into()
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

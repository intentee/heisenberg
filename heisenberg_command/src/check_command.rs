use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ffi::OsString;

use clap::Args;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use heisenberg_error_context::error_context::error_context;
use heisenberg_module_cycles::module_cycles::ModuleCycles;
use heisenberg_module_dependencies::module_dependencies::ModuleDependencies;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_module_graph::module_findings::ModuleFindings;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_project_model::clippy_requirements::ClippyRequirements;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::loaded_project::LoadedProject;
use heisenberg_project_model::manifest_lint_allowances::ManifestLintAllowances;
use heisenberg_project_model::nextest_requirements::NextestRequirements;
use heisenberg_project_model::package_rules::PackageRules;
use heisenberg_project_model::package_violation::PackageViolation;
use heisenberg_project_model::resolved_project::ResolvedProject;

use crate::check_report::CheckReport;
use crate::configuration_analysis::ConfigurationAnalysis;
use crate::configuration_pipeline::ConfigurationPipeline;
use crate::heisenberg_command_error::HeisenbergCommandError;
use crate::project_arguments::ProjectArguments;

fn empty_libraries(analyses: &[ConfigurationAnalysis]) -> BTreeSet<String> {
    let mut libraries: BTreeMap<&str, bool> = BTreeMap::new();

    for CrateModuleGraph {
        crate_target,
        root_module_is_empty,
        ..
    } in analyses
        .iter()
        .flat_map(|ConfigurationAnalysis { graphs, .. }| graphs)
        .filter(|CrateModuleGraph { crate_target, .. }| {
            crate_target.kind == CrateTargetKind::Library
        })
    {
        *libraries
            .entry(crate_target.package_name.as_str())
            .or_insert(true) &= root_module_is_empty;
    }

    libraries
        .into_iter()
        .filter(|(_, is_empty)| *is_empty)
        .map(|(package, _)| package.to_owned())
        .collect()
}

fn helper_holding_packages(analyses: &[ConfigurationAnalysis]) -> BTreeSet<String> {
    analyses
        .iter()
        .flat_map(|ConfigurationAnalysis { graphs, .. }| graphs)
        .filter(
            |CrateModuleGraph {
                 crate_target,
                 findings: ModuleFindings { violations, .. },
                 ..
             }| {
                violations.iter().any(|(Violation { kind, .. }, _)| {
                    (crate_target.kind == CrateTargetKind::IntegrationTest
                        && *kind == ViolationKind::NonTestItemInTestTarget)
                        || (crate_target.kind == CrateTargetKind::Benchmark
                            && *kind == ViolationKind::NonBenchmarkItemInBenchmarkTarget)
                })
            },
        )
        .map(|CrateModuleGraph { crate_target, .. }| crate_target.package_name.clone())
        .collect()
}

#[derive(Args, Debug)]
pub struct CheckCommand {
    #[command(flatten)]
    pub project: ProjectArguments,
}

impl CheckCommand {
    /// # Errors
    ///
    /// Returns every [`HeisenbergCommandError`] that kept the project from loading or a
    /// declared configuration from being analysed.
    pub fn execute(
        &self,
        environment: &BTreeMap<OsString, OsString>,
    ) -> Result<CheckReport, Vec<HeisenbergCommandError>> {
        self.run(&self.project.model(environment))
    }
}

impl ConfigurationPipeline for CheckCommand {
    type Analysis = ConfigurationAnalysis;
    type Prerequisite = Vec<PackageViolation>;
    type Report = CheckReport;

    fn analysis(
        &self,
        configuration: &Configuration,
        project: &mut LoadedProject<'_>,
        _: &Self::Prerequisite,
    ) -> Result<Self::Analysis, HeisenbergCommandError> {
        project
            .packages
            .confinement
            .reach(&project.build)
            .map_err(Box::new)
            .map_err(error_context(
                configuration.clone(),
                HeisenbergCommandError::ConfigurationProjectModel,
            ))
            .and_then(|confinement| {
                ModuleDependencies { project }
                    .extract()
                    .map_err(error_context(
                        configuration.clone(),
                        HeisenbergCommandError::ModuleDependencies,
                    ))
                    .map(|graphs| ConfigurationAnalysis {
                        configuration: configuration.clone(),
                        confinement,
                        cycles: graphs
                            .par_iter()
                            .flat_map_iter(|graph| ModuleCycles { graph }.find())
                            .collect(),
                        graphs,
                    })
            })
    }

    fn prerequisite(
        &self,
        ResolvedProject { project, .. }: &ResolvedProject<'_>,
    ) -> Result<Self::Prerequisite, HeisenbergCommandError> {
        ClippyRequirements {
            resolved: &project.resolved,
        }
        .violations()
        .and_then(|clippy_violations| {
            NextestRequirements {
                resolved: &project.resolved,
            }
            .violations()
            .map(|nextest_violations| {
                clippy_violations
                    .into_iter()
                    .chain(nextest_violations)
                    .collect()
            })
        })
        .map_err(HeisenbergCommandError::ProjectModel)
    }

    fn report(
        &self,
        ResolvedProject {
            declared_test_crates,
            packages,
            project,
            scope,
            ..
        }: &ResolvedProject<'_>,
        analyses: Vec<Self::Analysis>,
        tool_violations: &Self::Prerequisite,
    ) -> Self::Report {
        let package_violations = tool_violations
            .iter()
            .cloned()
            .chain(declared_test_crates.violations(&project.resolved, &packages.test_crates))
            .chain(packages.confinement.violations())
            .chain(if scope.is_partial() {
                Vec::new()
            } else {
                packages.confinement.unreached_users(
                    analyses
                        .iter()
                        .map(|ConfigurationAnalysis { confinement, .. }| confinement),
                )
            })
            .chain(
                ManifestLintAllowances {
                    resolved: &project.resolved,
                }
                .violations(),
            )
            .chain(
                PackageRules {
                    empty_libraries: &if scope.is_partial() {
                        BTreeSet::new()
                    } else {
                        empty_libraries(&analyses)
                    },
                    helper_holding_packages: &helper_holding_packages(&analyses),
                    resolved: &project.resolved,
                }
                .violations(),
            )
            .collect();

        CheckReport {
            analyses,
            package_violations,
            scope: scope.clone(),
        }
    }
}

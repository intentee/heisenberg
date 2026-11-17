use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ffi::OsString;

use clap::Args;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use heisenberg_module_cycles::module_cycles::ModuleCycles;
use heisenberg_module_dependencies::module_dependencies::ModuleDependencies;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_module_graph::module_findings::ModuleFindings;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_project_model::clippy_requirements::ClippyRequirements;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::dependency_requirements::DependencyRequirements;
use heisenberg_project_model::dependency_roles::DependencyRoles;
use heisenberg_project_model::loaded_project::LoadedProject;
use heisenberg_project_model::manifest_lint_allowances::ManifestLintAllowances;
use heisenberg_project_model::nextest_requirements::NextestRequirements;
use heisenberg_project_model::nextest_review::NextestReview;
use heisenberg_project_model::package_rules::PackageRules;
use heisenberg_project_model::resolved_project::ResolvedProject;
use heisenberg_project_model::run_scope::RunScope;
use heisenberg_project_model::rust_versions::RustVersions;
use heisenberg_project_model::rustfmt_requirements::RustfmtRequirements;

use crate::check_report::CheckReport;
use crate::configuration_analysis::ConfigurationAnalysis;
use crate::configuration_pipeline::ConfigurationPipeline;
use crate::dependency_uses::dependency_uses;
use crate::heisenberg_command_error::HeisenbergCommandError;
use crate::nextest_presence_verdict::NextestPresenceVerdict;
use crate::project_arguments::ProjectArguments;
use crate::tool_review::ToolReview;
use crate::unreachable_source_files::unreachable_source_files;

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
    ) -> Result<CheckReport, HeisenbergCommandError> {
        self.run(&self.project.model(environment))
    }
}

impl ConfigurationPipeline for CheckCommand {
    type Analysis = ConfigurationAnalysis;
    type Prerequisite = ToolReview;
    type Report = CheckReport;

    fn analysis(
        &self,
        configuration: &Configuration,
        project: &mut LoadedProject<'_>,
        scope: &RunScope,
        _: &Self::Prerequisite,
    ) -> Result<Self::Analysis, HeisenbergCommandError> {
        let member_source_files = if scope.is_partial() {
            BTreeSet::new()
        } else {
            project
                .files
                .member_sources(&project.packages.member_directories)
        };
        let build_script_output_files = if scope.is_partial() {
            BTreeSet::new()
        } else {
            project
                .files
                .relative_paths(&project.build_script_output_files)
        };
        let confinement = project.packages.confinement.reach(&project.build);

        ModuleDependencies { project }
            .extract()
            .map_err(|source| HeisenbergCommandError::ModuleDependencies {
                configuration: configuration.clone(),
                source: Box::new(source),
            })
            .map(|graphs| ConfigurationAnalysis {
                build_script_output_files,
                configuration: configuration.clone(),
                confinement,
                cycles: graphs
                    .par_iter()
                    .flat_map_iter(|graph| ModuleCycles { graph }.find())
                    .collect(),
                graphs,
                member_source_files,
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
            .review()
            .map(|nextest| ToolReview {
                clippy_violations,
                nextest,
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
        ToolReview {
            clippy_violations,
            nextest:
                NextestReview {
                    presence,
                    violations: nextest_violations,
                },
        }: &Self::Prerequisite,
    ) -> Self::Report {
        let nextest_presence = NextestPresenceVerdict::new(presence, &analyses, scope);
        let module_files: BTreeSet<&String> = analyses
            .iter()
            .flat_map(|ConfigurationAnalysis { graphs, .. }| graphs)
            .flat_map(|CrateModuleGraph { module_files, .. }| module_files)
            .collect();
        let package_violations = clippy_violations
            .iter()
            .chain(nextest_violations)
            .cloned()
            .chain(declared_test_crates.violations(&project.resolved, &packages.test_crates))
            .chain(packages.confinement.violations())
            .chain(if scope.is_partial() {
                Vec::new()
            } else {
                unreachable_source_files(&analyses)
            })
            .chain(if scope.is_partial() {
                Vec::new()
            } else {
                packages.confinement.unreached_users(
                    analyses
                        .iter()
                        .map(|ConfigurationAnalysis { confinement, .. }| confinement),
                )
            })
            .chain(if scope.is_partial() {
                Vec::new()
            } else {
                DependencyRoles {
                    resolved: &project.resolved,
                    test_crates: &packages.test_crates,
                    uses: &dependency_uses(&analyses),
                }
                .violations()
            })
            .chain(
                ManifestLintAllowances {
                    resolved: &project.resolved,
                }
                .violations(),
            )
            .chain(
                DependencyRequirements {
                    resolved: &project.resolved,
                }
                .violations(),
            )
            .chain(
                RustVersions {
                    resolved: &project.resolved,
                }
                .violations(),
            )
            .chain(
                RustfmtRequirements {
                    module_files: &module_files,
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
            nextest_presence,
            package_violations,
            scope: scope.clone(),
        }
    }
}

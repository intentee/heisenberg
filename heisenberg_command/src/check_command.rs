use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ffi::OsString;
use std::num::NonZeroU16;
use std::sync::Arc;
use std::thread::scope;

use clap::Args;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use heisenberg_absolute_path::absolute_path::AbsolutePath;
use heisenberg_analysis_threads::analysis_thread::AnalysisThread;
use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
use heisenberg_analysis_threads::worker_pool::WorkerPool;
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
use heisenberg_project_model::configuration_selection::ConfigurationSelection;
use heisenberg_project_model::constants::CARGO_MANIFEST_FILE;
use heisenberg_project_model::loaded_project::LoadedProject;
use heisenberg_project_model::loaded_workspace::LoadedWorkspace;
use heisenberg_project_model::manifest_lint_allowances::ManifestLintAllowances;
use heisenberg_project_model::nextest_requirements::NextestRequirements;
use heisenberg_project_model::package_rules::PackageRules;
use heisenberg_project_model::package_violation::PackageViolation;
use heisenberg_project_model::project_model::ProjectModel;
use heisenberg_project_model::project_model_error::ProjectModelError;
use heisenberg_project_model::resolved_project::ResolvedProject;
use heisenberg_project_model::target_model::TargetModel;
use heisenberg_workspace_build::build_outputs::BuildOutputs;

use crate::check_report::CheckReport;
use crate::configuration_analysis::ConfigurationAnalysis;
use crate::constants::DEFAULT_JOBS;
use crate::constants::SUPPORTED_RUST_VERSION;
use crate::heisenberg_command_error::HeisenbergCommandError;

fn project_model_failure(error: ProjectModelError) -> Vec<HeisenbergCommandError> {
    vec![HeisenbergCommandError::ProjectModel(error)]
}

fn loaded<'resolved>(
    project: &'resolved ResolvedProject<'_>,
    configuration: &Configuration,
    target: &Arc<TargetModel>,
) -> Result<LoadedWorkspace<'resolved>, HeisenbergCommandError> {
    project
        .load(configuration, target)
        .map_err(Box::new)
        .map_err(error_context(
            configuration.clone(),
            HeisenbergCommandError::ConfigurationProjectModel,
        ))
}

fn analysed(
    configuration: &Configuration,
    workspace: LoadedWorkspace<'_>,
    outputs: &BuildOutputs,
) -> Result<ConfigurationAnalysis, HeisenbergCommandError> {
    workspace
        .built(outputs)
        .map_err(Box::new)
        .map_err(error_context(
            configuration.clone(),
            HeisenbergCommandError::ConfigurationProjectModel,
        ))
        .and_then(|mut loaded| {
            let graphs = ModuleDependencies {
                project: &mut loaded,
            }
            .extract()
            .map_err(error_context(
                configuration.clone(),
                HeisenbergCommandError::ModuleDependencies,
            ));
            let LoadedProject { database, .. } = loaded;

            WorkerPool::release(database);

            graphs
        })
        .map(|graphs| ConfigurationAnalysis {
            configuration: configuration.clone(),
            cycles: graphs
                .par_iter()
                .flat_map_iter(|graph| ModuleCycles { graph }.find())
                .collect(),
            graphs,
        })
}

fn configuration_thread<'scope, TResult: Send + 'scope>(
    threads: &AnalysisThreads<'scope, '_>,
    configuration: &Configuration,
    work: impl FnOnce() -> Result<TResult, HeisenbergCommandError> + Send + 'scope,
) -> Result<AnalysisThread<'scope, Result<TResult, HeisenbergCommandError>>, HeisenbergCommandError>
{
    threads.spawn(work).map_err(error_context(
        configuration.clone(),
        HeisenbergCommandError::AnalysisThread,
    ))
}

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

fn every_analysis(
    outcomes: Vec<Result<ConfigurationAnalysis, HeisenbergCommandError>>,
) -> Result<Vec<ConfigurationAnalysis>, Vec<HeisenbergCommandError>> {
    if outcomes.iter().all(Result::is_ok) {
        Ok(outcomes.into_iter().flatten().collect())
    } else {
        Err(outcomes.into_iter().filter_map(Result::err).collect())
    }
}

fn joined<TResult>(
    running: Vec<
        Result<AnalysisThread<'_, Result<TResult, HeisenbergCommandError>>, HeisenbergCommandError>,
    >,
) -> Vec<Result<TResult, HeisenbergCommandError>> {
    running
        .into_iter()
        .map(|analysis| analysis.and_then(AnalysisThread::join))
        .collect()
}

fn report(
    ResolvedProject {
        declared_test_crates,
        packages,
        project,
        scope,
        ..
    }: &ResolvedProject<'_>,
    analyses: Vec<ConfigurationAnalysis>,
    tool_violations: Vec<PackageViolation>,
) -> CheckReport {
    let package_violations = tool_violations
        .into_iter()
        .chain(declared_test_crates.violations(&project.resolved, &packages.test_crates))
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
                resolved: &project.resolved,
                test_library_packages: &test_library_packages(&analyses),
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

fn test_library_packages(analyses: &[ConfigurationAnalysis]) -> BTreeSet<String> {
    analyses
        .iter()
        .flat_map(|ConfigurationAnalysis { graphs, .. }| graphs)
        .filter(
            |CrateModuleGraph {
                 crate_target,
                 findings: ModuleFindings { violations, .. },
                 ..
             }| {
                crate_target.kind == CrateTargetKind::IntegrationTest
                    && violations.iter().any(|(Violation { kind, .. }, _)| {
                        *kind == ViolationKind::NonTestItemInTestTarget
                    })
            },
        )
        .map(|CrateModuleGraph { crate_target, .. }| crate_target.package_name.clone())
        .collect()
}

#[derive(Args, Debug)]
pub struct CheckCommand {
    #[arg(
        long,
        env = "CARGO_HOME",
        hide_env_values = true,
        value_parser = AbsolutePath::parse,
        help = "Absolute path of the Cargo home directory holding the downloaded dependency sources"
    )]
    pub cargo_home: AbsolutePath,
    #[arg(
        long = "configuration",
        value_name = "NAME",
        required = true,
        help = "Analyse this declared configuration; repeat it to select several"
    )]
    pub configurations: Vec<String>,
    #[arg(
        long,
        default_value_t = DEFAULT_JOBS,
        help = "Number of analysis worker threads, cargo jobs and proc-macro server processes"
    )]
    pub jobs: NonZeroU16,
    #[arg(
        long,
        value_parser = AbsolutePath::parse,
        help = format!("Absolute path of the Rust {SUPPORTED_RUST_VERSION} sysroot that contains the rust-src component")
    )]
    pub sysroot: AbsolutePath,
    #[arg(
        long,
        value_parser = AbsolutePath::parse,
        help = format!("Absolute path of the Cargo workspace directory that contains {CARGO_MANIFEST_FILE}")
    )]
    pub workspace: AbsolutePath,
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
        WorkerPool::start(self.jobs)
            .map_err(|error| vec![HeisenbergCommandError::WorkerPool(error)])?;

        let model = ProjectModel {
            cargo_home: self.cargo_home.path.clone(),
            configurations: ConfigurationSelection {
                names: self.configurations.clone(),
            },
            environment: environment.clone(),
            jobs: self.jobs,
            sysroot: self.sysroot.path.clone(),
            workspace_directory: self.workspace.path.clone(),
        };
        let project = model.resolve().map_err(project_model_failure)?;
        let project = &project;
        let targets = project.target_models();

        scope(|scope| {
            let threads = AnalysisThreads { scope };
            let loading = targets
                .iter()
                .flatten()
                .map(|(configuration, target)| {
                    let owned_configuration = configuration.clone();
                    let target = Arc::clone(target);

                    configuration_thread(&threads, configuration, move || {
                        loaded(project, &owned_configuration, &target)
                    })
                })
                .collect();
            let builds = project.build_outputs();
            let tool_violations = ClippyRequirements {
                resolved: &project.project.resolved,
            }
            .violations()
            .and_then(|clippy_violations| {
                NextestRequirements {
                    resolved: &project.project.resolved,
                }
                .violations()
                .map(|nextest_violations| {
                    clippy_violations
                        .into_iter()
                        .chain(nextest_violations)
                        .collect::<Vec<PackageViolation>>()
                })
            });
            let loads = joined(loading);

            builds
                .and_then(|outputs| targets.map(|_| outputs))
                .map_err(project_model_failure)
                .and_then(|outputs| {
                    tool_violations
                        .map_err(project_model_failure)
                        .and_then(|tool_violations| {
                            every_analysis(joined(
                                outputs
                                    .into_iter()
                                    .zip(loads)
                                    .map(|((configuration, build), load)| {
                                        load.and_then(|workspace| {
                                            configuration_thread(
                                                &threads,
                                                &configuration.clone(),
                                                move || analysed(&configuration, workspace, &build),
                                            )
                                        })
                                    })
                                    .collect(),
                            ))
                            .map(|analyses| report(project, analyses, tool_violations))
                        })
                })
        })
    }
}

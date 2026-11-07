use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::convert::identity;
use std::ffi::OsString;
use std::thread::scope;

use clap::Args;

use heisenberg_absolute_path::absolute_path::AbsolutePath;
use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
use heisenberg_analysis_threads::panic_payload::PanicPayload;
use heisenberg_error_context::error_context::error_context;
use heisenberg_module_cycles::module_cycles::ModuleCycles;
use heisenberg_module_dependencies::module_dependencies::ModuleDependencies;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_module_graph::violation::Violation;
use heisenberg_module_graph::violation_kind::ViolationKind;
use heisenberg_project_model::clippy_requirements::ClippyRequirements;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::configuration_selection::ConfigurationSelection;
use heisenberg_project_model::constants::CARGO_MANIFEST_FILE;
use heisenberg_project_model::manifest_lint_allowances::ManifestLintAllowances;
use heisenberg_project_model::package_rules::PackageRules;
use heisenberg_project_model::project_model::ProjectModel;
use heisenberg_project_model::project_model_error::ProjectModelError;
use heisenberg_project_model::resolved_project::ResolvedProject;

use crate::check_report::CheckReport;
use crate::configuration_analysis::ConfigurationAnalysis;
use crate::constants::SUPPORTED_RUST_VERSION;
use crate::heisenberg_command_error::HeisenbergCommandError;
use crate::running_analysis::RunningAnalysis;

fn project_model_failure(error: ProjectModelError) -> Vec<HeisenbergCommandError> {
    vec![HeisenbergCommandError::ProjectModel(error)]
}

fn analysed(
    project: &ResolvedProject,
    configuration: &Configuration,
) -> Result<ConfigurationAnalysis, HeisenbergCommandError> {
    project
        .load(configuration)
        .map_err(Box::new)
        .map_err(error_context(
            configuration.clone(),
            HeisenbergCommandError::ConfigurationProjectModel,
        ))
        .and_then(|mut loaded| {
            ModuleDependencies {
                project: &mut loaded,
            }
            .extract()
            .map_err(error_context(
                configuration.clone(),
                HeisenbergCommandError::ModuleDependencies,
            ))
        })
        .map(|graphs| ConfigurationAnalysis {
            configuration: configuration.clone(),
            cycles: graphs
                .iter()
                .flat_map(|graph| ModuleCycles { graph }.find())
                .collect(),
            graphs,
        })
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

fn joined(
    running: Vec<Result<RunningAnalysis<'_>, HeisenbergCommandError>>,
) -> Vec<Result<ConfigurationAnalysis, HeisenbergCommandError>> {
    running
        .into_iter()
        .map(|analysis| {
            analysis.and_then(
                |RunningAnalysis {
                     configuration,
                     handle,
                 }| {
                    handle
                        .join()
                        .map_err(PanicPayload)
                        .map_err(error_context(
                            configuration,
                            HeisenbergCommandError::AnalysisPanicked,
                        ))
                        .and_then(identity)
                },
            )
        })
        .collect()
}

fn test_library_packages(analyses: &[ConfigurationAnalysis]) -> BTreeSet<String> {
    analyses
        .iter()
        .flat_map(|ConfigurationAnalysis { graphs, .. }| graphs)
        .filter(
            |CrateModuleGraph {
                 crate_target,
                 violations,
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
        let project = ProjectModel {
            cargo_home: self.cargo_home.path.clone(),
            configurations: ConfigurationSelection {
                names: self.configurations.clone(),
            },
            environment: environment.clone(),
            sysroot: self.sysroot.path.clone(),
            workspace_directory: self.workspace.path.clone(),
        }
        .resolve()
        .map_err(project_model_failure)?;
        let clippy_violations = ClippyRequirements {
            resolved: &project.project.resolved,
            test_crates: &project.packages.test_crates,
        }
        .violations()
        .map_err(project_model_failure)?;
        let project = &project;

        every_analysis(scope(|scope| {
            let threads = AnalysisThreads { scope };

            joined(
                project
                    .build_outputs
                    .keys()
                    .map(|configuration| {
                        threads
                            .spawn(move || analysed(project, configuration))
                            .map_err(error_context(
                                configuration.clone(),
                                HeisenbergCommandError::AnalysisThread,
                            ))
                            .map(|handle| RunningAnalysis {
                                configuration: configuration.clone(),
                                handle,
                            })
                    })
                    .collect(),
            )
        }))
        .map(|analyses| {
            let package_violations = clippy_violations
                .into_iter()
                .chain(
                    project
                        .declared_test_crates
                        .violations(&project.project.resolved, &project.packages.test_crates),
                )
                .chain(
                    ManifestLintAllowances {
                        resolved: &project.project.resolved,
                    }
                    .violations(),
                )
                .chain(
                    PackageRules {
                        empty_libraries: &if project.scope.is_partial() {
                            BTreeSet::new()
                        } else {
                            empty_libraries(&analyses)
                        },
                        resolved: &project.project.resolved,
                        test_library_packages: &test_library_packages(&analyses),
                    }
                    .violations(),
                )
                .collect();

            CheckReport {
                analyses,
                package_violations,
                scope: project.scope.clone(),
            }
        })
    }
}

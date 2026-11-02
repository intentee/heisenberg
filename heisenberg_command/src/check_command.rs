use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::convert::identity;
use std::ffi::OsString;
use std::thread::scope;

use clap::Args;
use itertools::Itertools;

use heisenberg_absolute_path::absolute_path::AbsolutePath;
use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
use heisenberg_analysis_threads::panic_payload::PanicPayload;
use heisenberg_error_context::error_context::error_context;
use heisenberg_module_cycles::module_cycles::ModuleCycles;
use heisenberg_module_dependencies::module_dependencies::ModuleDependencies;
use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::crate_target_kind::CrateTargetKind;
use heisenberg_project_model::clippy_requirements::ClippyRequirements;
use heisenberg_project_model::constants::CARGO_MANIFEST_FILE;
use heisenberg_project_model::manifest_lint_allowances::ManifestLintAllowances;
use heisenberg_project_model::package_rules::PackageRules;
use heisenberg_project_model::project_model::ProjectModel;
use heisenberg_project_model::project_model_error::ProjectModelError;
use heisenberg_project_model::resolved_project::ResolvedProject;
use heisenberg_project_model::supported_target::SupportedTarget;

use crate::check_report::CheckReport;
use crate::constants::SUPPORTED_RUST_VERSION;
use crate::heisenberg_command_error::HeisenbergCommandError;
use crate::running_analysis::RunningAnalysis;
use crate::target_analysis::TargetAnalysis;

fn project_model_failure(error: ProjectModelError) -> Vec<HeisenbergCommandError> {
    vec![HeisenbergCommandError::ProjectModel(error)]
}

fn analysed(
    project: &ResolvedProject,
    target: SupportedTarget,
) -> Result<TargetAnalysis, HeisenbergCommandError> {
    project
        .load(target)
        .map_err(Box::new)
        .map_err(error_context(
            target,
            HeisenbergCommandError::TargetProjectModel,
        ))
        .and_then(|mut loaded| {
            ModuleDependencies {
                project: &mut loaded,
            }
            .extract()
            .map_err(error_context(
                target,
                HeisenbergCommandError::ModuleDependencies,
            ))
        })
        .map(|graphs| TargetAnalysis {
            cycles: graphs
                .iter()
                .flat_map(|graph| ModuleCycles { graph }.find())
                .collect(),
            graphs,
            target,
        })
}

fn empty_libraries(analyses: &[TargetAnalysis]) -> BTreeSet<String> {
    let mut libraries: BTreeMap<&str, bool> = BTreeMap::new();

    for CrateModuleGraph {
        crate_target,
        root_module_is_empty,
        ..
    } in analyses
        .iter()
        .flat_map(|TargetAnalysis { graphs, .. }| graphs)
        .filter(|graph| graph.crate_target.kind == CrateTargetKind::Library)
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

fn joined(
    running: Vec<Result<RunningAnalysis<'_>, HeisenbergCommandError>>,
) -> Vec<Result<TargetAnalysis, HeisenbergCommandError>> {
    running
        .into_iter()
        .map(|analysis| {
            analysis.and_then(|RunningAnalysis { handle, target }| {
                handle
                    .join()
                    .map_err(PanicPayload)
                    .map_err(error_context(
                        target,
                        HeisenbergCommandError::AnalysisPanicked,
                    ))
                    .and_then(identity)
            })
        })
        .collect()
}

#[derive(Args, Debug)]
pub struct CheckCommand {
    #[arg(
        long,
        env = "CARGO_HOME",
        value_parser = AbsolutePath::parse,
        help = "Absolute path of the Cargo home directory holding the downloaded dependency sources"
    )]
    pub cargo_home: AbsolutePath,
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
    /// supported target from being analysed.
    pub fn execute(
        &self,
        environment: &BTreeMap<OsString, OsString>,
    ) -> Result<CheckReport, Vec<HeisenbergCommandError>> {
        let project = ProjectModel {
            cargo_home: self.cargo_home.path.clone(),
            environment: environment.clone(),
            sysroot: self.sysroot.path.clone(),
            workspace_directory: self.workspace.path.clone(),
        }
        .resolve()
        .map_err(project_model_failure)?;
        let clippy_violations = ClippyRequirements {
            resolved: &project.project.resolved,
        }
        .violations()
        .map_err(project_model_failure)?;
        let project = &project;
        let (analyses, failures): (Vec<TargetAnalysis>, Vec<HeisenbergCommandError>) =
            scope(|scope| {
                let threads = AnalysisThreads { scope };

                joined(
                    project
                        .targets
                        .iter()
                        .copied()
                        .map(|target| {
                            threads
                                .spawn(move || analysed(project, target))
                                .map_err(error_context(
                                    target,
                                    HeisenbergCommandError::AnalysisThread,
                                ))
                                .map(|handle| RunningAnalysis { handle, target })
                        })
                        .collect(),
                )
            })
            .into_iter()
            .partition_result();

        failures
            .is_empty()
            .then_some(analyses)
            .ok_or(failures)
            .map(|analyses| {
                let package_violations = clippy_violations
                    .into_iter()
                    .chain(
                        ManifestLintAllowances {
                            resolved: &project.project.resolved,
                        }
                        .violations(),
                    )
                    .chain(
                        PackageRules {
                            empty_libraries: &empty_libraries(&analyses),
                            resolved: &project.project.resolved,
                        }
                        .violations(),
                    )
                    .collect();

                CheckReport {
                    analyses,
                    package_violations,
                }
            })
    }
}

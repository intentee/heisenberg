use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::num::NonZeroU16;
use std::sync::OnceLock;
use std::thread::scope;

use ra_ap_ide_db::base_db::Env;
use ra_ap_paths::AbsPathBuf;
use ra_ap_project_model::ManifestPath;

use heisenberg_analysis_threads::analysis_thread::AnalysisThread;
use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
use heisenberg_error_context::error_context::error_context;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_task_graph::once_map::OnceMap;

use crate::cargo_packages::CargoPackages;
use crate::cargo_workspace_role::CargoWorkspaceRole;
use crate::configuration_selection::ConfigurationSelection;
use crate::declared_configurations::DeclaredConfigurations;
use crate::declared_out_dirs::DeclaredOutDirs;
use crate::declared_test_crates::DeclaredTestCrates;
use crate::feature_graph::FeatureGraph;
use crate::heisenberg_settings::HeisenbergSettings;
use crate::library_directory::library_directory;
use crate::member_target::MemberTarget;
use crate::project_model_error::ProjectModelError;
use crate::resolved_packages::ResolvedPackages;
use crate::resolved_project::ResolvedProject;
use crate::run_context::RunContext;
use crate::run_scope::RunScope;
use crate::utf8_environment::utf8_environment;
use crate::workspace_manifest::workspace_manifest;
use crate::workspace_packages::WorkspacePackages;

fn cargo_environment(variables: &BTreeMap<String, String>) -> Env {
    let mut environment = Env::default();

    for (name, value) in variables {
        environment.set(name, value.clone());
    }

    environment
}

fn validate_crate_roots(
    ResolvedPackages {
        member_targets,
        workspace,
        ..
    }: &ResolvedPackages,
) -> Result<(), ProjectModelError> {
    let mut owners: BTreeMap<AbsPathBuf, &CrateTarget> = BTreeMap::new();

    for MemberTarget { crate_target, root } in member_targets {
        let crate_root = root.normalize();

        if !crate_root.starts_with(&workspace.root) {
            return Err(ProjectModelError::CrateRootOutsideWorkspace {
                crate_root: crate_root.into(),
                workspace_root: workspace.root.clone().into(),
            });
        }

        if let Some(owner) = owners.insert(crate_root.clone(), crate_target) {
            return Err(ProjectModelError::SharedCrateRoot {
                crate_root: crate_root.into(),
                first: Box::new(owner.clone()),
                second: Box::new(crate_target.clone()),
            });
        }

        fs::read(root).map_err(error_context(
            root.clone().into(),
            ProjectModelError::CrateRootUnreadable,
        ))?;
    }

    Ok(())
}

pub struct ProjectModel {
    pub cargo_home: AbsPathBuf,
    pub configurations: ConfigurationSelection,
    pub environment: BTreeMap<OsString, OsString>,
    pub jobs: NonZeroU16,
    pub sysroot: AbsPathBuf,
    pub workspace_directory: AbsPathBuf,
}

impl ProjectModel {
    /// # Errors
    ///
    /// Returns [`ProjectModelError`] when the environment, the cargo workspace, its
    /// dependencies, its crate roots or the standard library cannot be resolved the way
    /// the analysis requires.
    pub fn resolve(&self) -> Result<ResolvedProject<'_>, ProjectModelError> {
        let run = RunContext {
            cargo_home: &self.cargo_home,
            environment: utf8_environment(&self.environment)?,
            jobs: self.jobs,
            sysroot: &self.sysroot,
            workspace_directory: &self.workspace_directory,
        };
        let library_environment = run.environment.clone();

        scope(|scope| {
            let library = AnalysisThreads { scope }.spawn(move || {
                CargoPackages {
                    cargo_home: self.cargo_home.as_ref(),
                    environment: &library_environment,
                    role: CargoWorkspaceRole::Library,
                    workspace_directory: &library_directory(&self.sysroot),
                }
                .resolve()
            });
            let project = CargoPackages {
                cargo_home: run.cargo_home.as_ref(),
                environment: &run.environment,
                role: CargoWorkspaceRole::Project,
                workspace_directory: run.workspace_directory,
            }
            .resolve()?;
            let manifest_path = workspace_manifest(run.workspace_directory);
            let HeisenbergSettings {
                configurations: declared_configurations,
                out_dirs,
                test_crates,
            } = HeisenbergSettings::of_workspace(&project.workspace.metadata, &manifest_path)?;
            let declared = DeclaredConfigurations {
                manifest_path: &manifest_path,
                resolved: &project,
            }
            .validated(declared_configurations)?;
            let configurations = self.configurations.analysed(&declared, &manifest_path)?;
            let substituted = DeclaredOutDirs {
                configurations: &declared,
                manifest_path: &manifest_path,
                resolved: &project,
            }
            .validated(out_dirs)?;

            validate_crate_roots(&project)?;

            let project_graph = FeatureGraph::new(project)
                .map_err(Box::new)
                .map_err(ProjectModelError::ProjectPackageGraph);

            library
                .map_err(ProjectModelError::LibraryResolutionThread)
                .and_then(AnalysisThread::join)
                .and_then(|library| {
                    project_graph.and_then(|project| {
                        FeatureGraph::new(library)
                            .map_err(Box::new)
                            .map_err(ProjectModelError::LibraryPackageGraph)
                            .and_then(|library| {
                                ManifestPath::try_from(manifest_path)
                                    .map_err(ProjectModelError::WorkspaceManifestPath)
                                    .map(|manifest| {
                                        let declared_test_crates =
                                            DeclaredTestCrates { names: test_crates };

                                        ResolvedProject {
                                            build_script_contents: OnceMap::default(),
                                            cargo_environment: cargo_environment(
                                                &project.resolved.configured_environment,
                                            ),
                                            configurations: substituted
                                                .into_iter()
                                                .filter(|(configuration, _)| {
                                                    configurations.contains(configuration)
                                                })
                                                .collect(),
                                            packages: WorkspacePackages::new(
                                                &project.resolved,
                                                &declared_test_crates,
                                            ),
                                            declared_test_crates,
                                            library,
                                            manifest,
                                            project,
                                            proc_macro_server: OnceLock::new(),
                                            run,
                                            scope: RunScope {
                                                left_out_configurations: declared
                                                    .difference(&configurations)
                                                    .cloned()
                                                    .collect(),
                                            },
                                            source_text_checks: OnceMap::default(),
                                        }
                                    })
                            })
                    })
                })
        })
    }
}

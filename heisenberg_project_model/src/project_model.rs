use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::sync::OnceLock;

use ra_ap_paths::AbsPathBuf;

use heisenberg_error_context::error_context::error_context;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_workspace_build::configured_context::ConfiguredContext;

use crate::cargo_packages::CargoPackages;
use crate::cargo_workspace_role::CargoWorkspaceRole;
use crate::configuration::Configuration;
use crate::configuration_builds::ConfigurationBuilds;
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
use crate::run_scope::RunScope;
use crate::supported_target::SupportedTarget;
use crate::target_model::TargetModel;
use crate::utf8_environment::utf8_environment;
use crate::workspace_manifest::workspace_manifest;
use crate::workspace_packages::WorkspacePackages;

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

fn target_models(
    configurations: &BTreeSet<Configuration>,
    library: &FeatureGraph,
    sysroot: &AbsPathBuf,
) -> Result<BTreeMap<SupportedTarget, TargetModel>, ProjectModelError> {
    configurations
        .iter()
        .map(|Configuration { target, .. }| *target)
        .collect::<BTreeSet<SupportedTarget>>()
        .into_iter()
        .map(|target| TargetModel::new(target, library, sysroot).map(|model| (target, model)))
        .collect()
}

pub struct ProjectModel {
    pub cargo_home: AbsPathBuf,
    pub configurations: ConfigurationSelection,
    pub environment: BTreeMap<OsString, OsString>,
    pub sysroot: AbsPathBuf,
    pub workspace_directory: AbsPathBuf,
}

impl ProjectModel {
    /// # Errors
    ///
    /// Returns [`ProjectModelError`] when the environment, the cargo workspace, its
    /// dependencies, its crate roots or the standard library cannot be resolved the way
    /// the analysis requires.
    pub fn resolve(&self) -> Result<ResolvedProject, ProjectModelError> {
        let environment = utf8_environment(&self.environment)?;
        let project = CargoPackages {
            cargo_home: self.cargo_home.as_ref(),
            environment: &environment,
            role: CargoWorkspaceRole::Project,
            workspace_directory: &self.workspace_directory,
        }
        .resolve()?;
        let manifest_path = workspace_manifest(&self.workspace_directory);
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

        let library = CargoPackages {
            cargo_home: self.cargo_home.as_ref(),
            environment: &environment,
            role: CargoWorkspaceRole::Library,
            workspace_directory: &library_directory(&self.sysroot),
        }
        .resolve()?;

        FeatureGraph::new(project)
            .map_err(Box::new)
            .map_err(ProjectModelError::ProjectPackageGraph)
            .and_then(|project| {
                FeatureGraph::new(library)
                    .map_err(Box::new)
                    .map_err(ProjectModelError::LibraryPackageGraph)
                    .and_then(|library| {
                        let build_outputs = ConfigurationBuilds {
                            configurations: &configurations,
                            context: &ConfiguredContext {
                                cargo_home: self.cargo_home.as_ref(),
                                environment: &environment,
                                nightly_features_allowed: false,
                                workspace_directory: self.workspace_directory.as_ref(),
                            },
                            manifest_path: &manifest_path,
                            resolved: &project.resolved,
                            substituted: &substituted,
                            sysroot: &self.sysroot,
                        }
                        .outputs();

                        build_outputs.and_then(|build_outputs| {
                            target_models(&configurations, &library, &self.sysroot).map(|targets| {
                                let declared_test_crates =
                                    DeclaredTestCrates { names: test_crates };

                                ResolvedProject {
                                    build_outputs,
                                    packages: WorkspacePackages::new(
                                        &project.resolved,
                                        &declared_test_crates,
                                    ),
                                    declared_test_crates,
                                    project,
                                    proc_macro_server: OnceLock::new(),
                                    scope: RunScope {
                                        left_out_configurations: declared
                                            .difference(&configurations)
                                            .cloned()
                                            .collect(),
                                    },
                                    targets,
                                }
                            })
                        })
                    })
            })
    }
}

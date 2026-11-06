use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;

use ra_ap_paths::AbsPathBuf;

use heisenberg_error_context::error_context::error_context;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_workspace_build::configured_context::ConfiguredContext;

use crate::cargo_packages::CargoPackages;
use crate::cargo_workspace_role::CargoWorkspaceRole;
use crate::constants::RUSTC_VARIABLE;
use crate::declared_out_dirs::DeclaredOutDirs;
use crate::declared_targets::declared_targets;
use crate::declared_test_crates::DeclaredTestCrates;
use crate::feature_graph::FeatureGraph;
use crate::feature_selection::FeatureSelection;
use crate::heisenberg_settings::HeisenbergSettings;
use crate::library_directory::library_directory;
use crate::member_target::MemberTarget;
use crate::project_model_error::ProjectModelError;
use crate::resolved_project::ResolvedProject;
use crate::run_scope::RunScope;
use crate::sysroot_compiler::sysroot_compiler;
use crate::target_builds::TargetBuilds;
use crate::target_selection::TargetSelection;
use crate::utf8_environment::utf8_environment;
use crate::workspace_manifest::workspace_manifest;

pub struct ProjectModel {
    pub cargo_home: AbsPathBuf,
    pub environment: BTreeMap<OsString, OsString>,
    pub features: FeatureSelection,
    pub sysroot: AbsPathBuf,
    pub targets: TargetSelection,
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
            out_dirs,
            targets: declared_triples,
            test_crates,
        } = HeisenbergSettings::of_workspace(&project.workspace.metadata, &manifest_path)?;
        let declared = declared_targets(declared_triples, &manifest_path)?;
        let targets = self.targets.analysed(&declared, &manifest_path)?;

        self.features.validate(&project, &manifest_path)?;

        let substituted = DeclaredOutDirs {
            manifest_path: &manifest_path,
            resolved: &project,
            targets: &declared,
        }
        .validated(out_dirs)?;
        let mut owners: BTreeMap<AbsPathBuf, &CrateTarget> = BTreeMap::new();

        for MemberTarget { crate_target, root } in &project.member_targets {
            let crate_root = root.normalize();

            if !crate_root.starts_with(&project.workspace.root) {
                return Err(ProjectModelError::CrateRootOutsideWorkspace {
                    crate_root: crate_root.into(),
                    workspace_root: project.workspace.root.clone().into(),
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
                        let compiler = sysroot_compiler(&self.sysroot);
                        let mut build_environment = environment.clone();

                        build_environment
                            .insert(RUSTC_VARIABLE.to_owned(), compiler.as_str().to_owned());

                        let build_outputs = TargetBuilds {
                            compiler: &compiler,
                            context: &ConfiguredContext {
                                cargo_home: self.cargo_home.as_ref(),
                                environment: &build_environment,
                                nightly_features_allowed: false,
                                workspace_directory: self.workspace_directory.as_ref(),
                            },
                            features: &self.features.cli_features(),
                            manifest_path: &manifest_path,
                            resolved: &project.resolved,
                            substituted: &substituted,
                            targets: &targets,
                        }
                        .outputs();

                        build_outputs.map(|build_outputs| ResolvedProject {
                            build_outputs,
                            declared_test_crates: DeclaredTestCrates { names: test_crates },
                            library,
                            project,
                            scope: RunScope {
                                features: self.features.clone(),
                                left_out_targets: declared.difference(&targets).copied().collect(),
                            },
                            sysroot: self.sysroot.clone(),
                            targets,
                        })
                    })
            })
    }
}

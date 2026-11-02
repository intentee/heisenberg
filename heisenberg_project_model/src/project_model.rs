use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;

use ra_ap_paths::AbsPathBuf;

use heisenberg_error_context::error_context::error_context;
use heisenberg_module_graph::crate_target::CrateTarget;

use crate::cargo_packages::CargoPackages;
use crate::cargo_workspace_role::CargoWorkspaceRole;
use crate::declared_targets::declared_targets;
use crate::feature_graph::FeatureGraph;
use crate::library_directory::library_directory;
use crate::member_target::MemberTarget;
use crate::project_model_error::ProjectModelError;
use crate::resolved_project::ResolvedProject;
use crate::utf8_environment::utf8_environment;
use crate::workspace_manifest::workspace_manifest;

pub struct ProjectModel {
    pub cargo_home: AbsPathBuf,
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
        let targets = declared_targets(
            &project.workspace.metadata,
            &workspace_manifest(&self.workspace_directory),
        )?;
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
                    .map(|library| ResolvedProject {
                        library,
                        project,
                        sysroot: self.sysroot.clone(),
                        targets,
                    })
            })
    }
}

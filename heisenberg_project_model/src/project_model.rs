use ra_ap_paths::AbsPathBuf;

use crate::cargo_feature_gate::CargoFeatureGate;
use crate::cargo_packages::CargoPackages;
use crate::feature_graph::FeatureGraph;
use crate::project_model_error::ProjectModelError;
use crate::resolved_project::ResolvedProject;

pub struct ProjectModel {
    pub cargo_home: AbsPathBuf,
    pub sysroot: AbsPathBuf,
    pub workspace_directory: AbsPathBuf,
}

impl ProjectModel {
    pub fn resolve(&self) -> Result<ResolvedProject, ProjectModelError> {
        let project = CargoPackages {
            cargo_feature_gate: CargoFeatureGate::Stable,
            cargo_home: self.cargo_home.as_ref(),
            workspace_directory: self.workspace_directory.as_ref(),
        }
        .resolve()?;
        let library_directory = self
            .sysroot
            .join("lib")
            .join("rustlib")
            .join("src")
            .join("rust")
            .join("library");
        let library = CargoPackages {
            cargo_feature_gate: CargoFeatureGate::Nightly,
            cargo_home: self.cargo_home.as_ref(),
            workspace_directory: library_directory.as_ref(),
        }
        .resolve()?;

        Ok(ResolvedProject {
            library: FeatureGraph::new(library),
            library_directory,
            project: FeatureGraph::new(project),
            sysroot: self.sysroot.clone(),
            workspace_directory: self.workspace_directory.clone(),
        })
    }
}

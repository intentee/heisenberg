use std::sync::Arc;

use guppy::graph::feature::StandardFeatures;
use guppy::platform::Platform;
use ra_ap_paths::AbsPathBuf;
use ra_ap_project_model::RustSourceWorkspaceConfig;
use ra_ap_project_model::Sysroot;

use crate::feature_graph::FeatureGraph;
use crate::library_directory::library_directory;
use crate::library_project_json::LibraryProjectJson;
use crate::project_model_error::ProjectModelError;
use crate::supported_target::SupportedTarget;
use crate::target_cfg::TargetCfg;

fn loaded_sysroot(
    library: &FeatureGraph,
    sysroot_directory: &AbsPathBuf,
    platform: &Arc<Platform>,
) -> Result<Sysroot, ProjectModelError> {
    let library_directory = library_directory(sysroot_directory);
    let mut sysroot = Sysroot::new(
        Some(sysroot_directory.clone()),
        Some(library_directory.clone()),
    );

    library
        .build(StandardFeatures::Default, platform)
        .map_err(Box::new)
        .map_err(ProjectModelError::LibraryFeatureResolution)
        .and_then(|build| {
            library
                .metadata(&build)
                .map_err(ProjectModelError::LibraryMetadata)
        })
        .and_then(|metadata| {
            LibraryProjectJson {
                library_directory: &library_directory,
                metadata: &metadata,
            }
            .project_json()
        })
        .and_then(|project_json| {
            sysroot
                .load_workspace(&RustSourceWorkspaceConfig::Json(project_json), false, &drop)
                .ok_or(ProjectModelError::LibraryWorkspaceMissing { library_directory })
        })
        .map(|library_workspace| {
            sysroot.set_workspace(library_workspace);

            sysroot
        })
}

pub struct TargetModel {
    pub(crate) cfg: TargetCfg,
    pub(crate) sysroot: Sysroot,
}

impl TargetModel {
    pub(crate) fn new(
        target: SupportedTarget,
        library: &FeatureGraph,
        sysroot_directory: &AbsPathBuf,
    ) -> Result<Self, ProjectModelError> {
        TargetCfg::parse(target).and_then(|cfg| {
            loaded_sysroot(library, sysroot_directory, &cfg.platform)
                .map(|sysroot| Self { cfg, sysroot })
        })
    }
}

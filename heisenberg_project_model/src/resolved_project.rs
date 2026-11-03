use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::sync::Arc;

use guppy::graph::feature::StandardFeatures;
use guppy::platform::Platform;
use ra_ap_cfg::CfgAtom;
use ra_ap_cfg::CfgDiff;
use ra_ap_ide_db::FxHashMap;
use ra_ap_ide_db::base_db::Env;
use ra_ap_intern::sym;
use ra_ap_load_cargo::LoadCargoConfig;
use ra_ap_load_cargo::ProcMacroServerChoice;
use ra_ap_load_cargo::load_workspace;
use ra_ap_paths::AbsPathBuf;
use ra_ap_project_model::CargoWorkspace;
use ra_ap_project_model::CfgOverrides;
use ra_ap_project_model::ManifestPath;
use ra_ap_project_model::ProjectWorkspace;
use ra_ap_project_model::ProjectWorkspaceKind;
use ra_ap_project_model::RustSourceWorkspaceConfig;
use ra_ap_project_model::Sysroot;
use ra_ap_project_model::WorkspaceBuildScripts;
use ra_ap_vfs::VfsPath;

use crate::disabled_proc_macros::DisabledProcMacros;
use crate::exact_crate_cfg::ExactCrateCfg;
use crate::feature_graph::FeatureGraph;
use crate::library_directory::library_directory;
use crate::library_project_json::LibraryProjectJson;
use crate::loaded_project::LoadedProject;
use crate::out_dir_environment::OutDirEnvironment;
use crate::project_model_error::ProjectModelError;
use crate::supported_target::SupportedTarget;
use crate::target_cfg::TargetCfg;
use crate::workspace_manifest::workspace_manifest;

pub struct ResolvedProject {
    pub library: FeatureGraph,
    pub out_dirs: BTreeMap<String, AbsPathBuf>,
    pub project: FeatureGraph,
    pub sysroot: AbsPathBuf,
    pub targets: BTreeSet<SupportedTarget>,
}

impl ResolvedProject {
    /// # Errors
    ///
    /// Returns [`ProjectModelError`] when the cfg tables, the metadata documents or the
    /// rust-analyzer workspace of `target` cannot be built.
    pub fn load(&self, target: SupportedTarget) -> Result<LoadedProject, ProjectModelError> {
        TargetCfg::parse(target).and_then(
            |TargetCfg {
                 crates,
                 library,
                 platform,
             }| {
                self.cargo_workspace(&platform)
                    .and_then(|cargo| {
                        self.sysroot(&platform)
                            .map(|sysroot| Self::project_workspace(cargo, crates, sysroot))
                    })
                    .and_then(|workspace| self.loaded(&workspace, &library, target))
            },
        )
    }

    fn cargo_workspace(
        &self,
        platform: &Arc<Platform>,
    ) -> Result<CargoWorkspace, ProjectModelError> {
        self.project
            .build(StandardFeatures::All, platform)
            .map_err(Box::new)
            .map_err(ProjectModelError::ProjectFeatureResolution)
            .and_then(|build| {
                self.project
                    .metadata(build)
                    .map_err(ProjectModelError::ProjectMetadata)
            })
            .and_then(|metadata| {
                ManifestPath::try_from(workspace_manifest(&self.project.resolved.workspace.root))
                    .map_err(ProjectModelError::WorkspaceManifestPath)
                    .map(|manifest| {
                        let mut configured = Env::default();

                        for (name, value) in &self.project.resolved.configured_environment {
                            configured.set(name, value.clone());
                        }

                        CargoWorkspace::new(metadata, manifest, configured, false)
                    })
            })
    }

    fn loaded(
        &self,
        workspace: &ProjectWorkspace,
        library_atoms: &[CfgAtom],
        target: SupportedTarget,
    ) -> Result<LoadedProject, ProjectModelError> {
        load_workspace(
            workspace.clone(),
            &FxHashMap::default(),
            &LoadCargoConfig {
                load_out_dirs_from_check: false,
                num_worker_threads: 1,
                prefill_caches: false,
                proc_macro_processes: 0,
                with_proc_macro_server: ProcMacroServerChoice::None,
            },
        )
        .map_err(Into::into)
        .map_err(ProjectModelError::WorkspaceLoad)
        .map(|(mut database, vfs, _)| {
            DisabledProcMacros {
                vfs: &vfs,
                workspace,
            }
            .apply(&mut database);

            let member_targets = self
                .project
                .resolved
                .member_targets
                .iter()
                .map(|member| {
                    (
                        VfsPath::from(member.root.clone()),
                        member.crate_target.clone(),
                    )
                })
                .collect::<HashMap<VfsPath, _>>();

            ExactCrateCfg {
                library_atoms,
                member_targets: &member_targets,
                vfs: &vfs,
            }
            .assign(&mut database);

            OutDirEnvironment {
                member_targets: &member_targets,
                out_dirs: &self.out_dirs,
                target,
                vfs: &vfs,
            }
            .assign(&mut database);

            LoadedProject {
                database,
                member_targets,
                vfs,
                workspace_root: self.project.resolved.workspace.root.clone(),
            }
        })
        .and_then(|loaded| {
            let virtual_file = loaded
                .vfs
                .iter()
                .map(|(_, path)| path)
                .find(|path| path.as_path().is_none())
                .cloned();

            virtual_file
                .map(ProjectModelError::VirtualSourceFile)
                .map_or(Ok(loaded), Err)
        })
    }

    fn project_workspace(
        cargo: CargoWorkspace,
        rustc_cfg: Vec<CfgAtom>,
        sysroot: Sysroot,
    ) -> ProjectWorkspace {
        ProjectWorkspace {
            cfg_overrides: CfgOverrides {
                global: CfgDiff::new(Vec::new(), vec![CfgAtom::Flag(sym::rust_analyzer)]),
                selective: FxHashMap::default(),
            },
            extra_includes: Vec::new(),
            kind: ProjectWorkspaceKind::Cargo {
                build_scripts: WorkspaceBuildScripts::default(),
                cargo,
                error: None,
                rustc: Err(None),
            },
            rustc_cfg,
            set_test: false,
            sysroot,
            target: Err("the target data layout is not used by name resolution".into()),
            toolchain: None,
        }
    }

    fn sysroot(&self, platform: &Arc<Platform>) -> Result<Sysroot, ProjectModelError> {
        let library_directory = library_directory(&self.sysroot);
        let mut sysroot = Sysroot::new(Some(self.sysroot.clone()), Some(library_directory.clone()));

        self.library
            .build(StandardFeatures::Default, platform)
            .map_err(Box::new)
            .map_err(ProjectModelError::LibraryFeatureResolution)
            .and_then(|build| {
                self.library
                    .metadata(build)
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
}

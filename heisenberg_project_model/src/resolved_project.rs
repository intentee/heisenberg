use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use cargo_metadata::Target;
use cargo_metadata::TargetKind;
use cargo_platform::Cfg;
use guppy::graph::feature::StandardFeatures;
use guppy::platform::Platform;
use ra_ap_cfg::CfgAtom;
use ra_ap_cfg::CfgDiff;
use ra_ap_ide_db::FxHashMap;
use ra_ap_ide_db::base_db::Env;
use ra_ap_ide_db::base_db::target::Arch;
use ra_ap_ide_db::base_db::target::TargetData;
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

use heisenberg_error_context::error_context::error_context;
use heisenberg_workspace_build::build_outputs::BuildOutputs;
use heisenberg_workspace_build::build_script_output::BuildScriptOutput;

use crate::build_script_environment::BuildScriptEnvironment;
use crate::cfg_atom::cfg_atom;
use crate::declared_test_crates::DeclaredTestCrates;
use crate::exact_crate_cfg::ExactCrateCfg;
use crate::feature_graph::FeatureGraph;
use crate::library_directory::library_directory;
use crate::library_project_json::LibraryProjectJson;
use crate::loaded_proc_macros::LoadedProcMacros;
use crate::loaded_project::LoadedProject;
use crate::member_target::MemberTarget;
use crate::out_dir_files::OutDirFiles;
use crate::package_document::PackageDocument;
use crate::project_model_error::ProjectModelError;
use crate::supported_target::SupportedTarget;
use crate::target_cfg::TargetCfg;
use crate::workspace_manifest::workspace_manifest;

fn build_script_atoms(
    crate_packages: &HashMap<VfsPath, String>,
    build_scripts: &BTreeMap<String, BuildScriptOutput>,
) -> Result<HashMap<VfsPath, Vec<CfgAtom>>, ProjectModelError> {
    crate_packages
        .iter()
        .filter_map(|(root, package)| {
            build_scripts
                .get(package)
                .map(|BuildScriptOutput { cfgs, .. }| {
                    cfgs.iter()
                        .map(|cfg| {
                            Cfg::from_str(cfg).map(cfg_atom).map_err(error_context(
                                cfg.clone(),
                                ProjectModelError::BuildScriptCfg,
                            ))
                        })
                        .collect::<Result<Vec<CfgAtom>, ProjectModelError>>()
                        .map(|atoms| (root.clone(), atoms))
                })
        })
        .collect()
}

pub struct ResolvedProject {
    pub build_outputs: BTreeMap<SupportedTarget, BuildOutputs>,
    pub declared_test_crates: DeclaredTestCrates,
    pub library: FeatureGraph,
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
                 data_layout,
                 library,
                 platform,
             }| {
                self.cargo_workspace(&platform)
                    .and_then(|cargo| {
                        self.sysroot(&platform).map(|sysroot| {
                            Self::project_workspace(cargo, crates, data_layout, sysroot)
                        })
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

    fn crate_packages(&self) -> HashMap<VfsPath, String> {
        self.project
            .resolved
            .packages
            .iter()
            .flat_map(|PackageDocument { metadata, .. }| {
                metadata
                    .targets
                    .iter()
                    .filter(|Target { kind, .. }| !kind.contains(&TargetKind::CustomBuild))
                    .map(|Target { src_path, .. }| {
                        (
                            VfsPath::from(AbsPathBuf::assert(src_path.clone())),
                            metadata.id.repr.clone(),
                        )
                    })
            })
            .collect()
    }

    fn loaded(
        &self,
        workspace: &ProjectWorkspace,
        library_atoms: &[CfgAtom],
        target: SupportedTarget,
    ) -> Result<LoadedProject, ProjectModelError> {
        let BuildOutputs {
            build_scripts,
            proc_macro_dylibs,
        } = &self.build_outputs[&target];
        let crate_packages = self.crate_packages();

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
        .and_then(|(mut database, mut vfs, _)| {
            build_script_atoms(&crate_packages, build_scripts)
                .and_then(|build_script_atoms| {
                    LoadedProcMacros {
                        dylibs: proc_macro_dylibs,
                        vfs: &vfs,
                        workspace,
                    }
                    .apply(&mut database)
                    .map(|()| build_script_atoms)
                })
                .and_then(|build_script_atoms| {
                    let member_targets = self
                        .project
                        .resolved
                        .member_targets
                        .iter()
                        .map(|MemberTarget { root, crate_target }| {
                            (VfsPath::from(root.clone()), crate_target.clone())
                        })
                        .collect::<HashMap<VfsPath, _>>();

                    ExactCrateCfg {
                        build_script_atoms: &build_script_atoms,
                        library_atoms,
                        member_targets: &member_targets,
                        vfs: &vfs,
                    }
                    .assign(&mut database);

                    BuildScriptEnvironment {
                        crate_packages: &crate_packages,
                        outputs: build_scripts,
                        vfs: &vfs,
                    }
                    .assign(&mut database);

                    OutDirFiles {
                        crate_packages: &crate_packages,
                        outputs: build_scripts,
                    }
                    .assign(&mut database, &mut vfs)
                    .map(|()| member_targets)
                })
                .map(|member_targets| LoadedProject {
                    database,
                    member_targets,
                    test_crates: self
                        .declared_test_crates
                        .test_crates(&self.project.resolved),
                    unpublished_packages: self
                        .project
                        .resolved
                        .members()
                        .filter(|member| member.is_unpublished())
                        .map(|PackageDocument { metadata, .. }| metadata.name.to_string())
                        .collect(),
                    vfs,
                    workspace_root: self.project.resolved.workspace.root.clone(),
                })
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
        data_layout: Box<str>,
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
            target: Ok(TargetData {
                data_layout,
                arch: Arch::Other,
            }),
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

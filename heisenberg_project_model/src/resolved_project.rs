use std::collections::BTreeMap;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::OnceLock;

use cargo_platform::Cfg;
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
use ra_ap_proc_macro_api::ProcMacroClient;
use ra_ap_project_model::CargoWorkspace;
use ra_ap_project_model::CfgOverrides;
use ra_ap_project_model::ManifestPath;
use ra_ap_project_model::ProjectWorkspace;
use ra_ap_project_model::ProjectWorkspaceKind;
use ra_ap_project_model::Sysroot;
use ra_ap_project_model::WorkspaceBuildScripts;
use ra_ap_vfs::VfsPath;

use heisenberg_error_context::error_context::error_context;
use heisenberg_workspace_build::build_outputs::BuildOutputs;
use heisenberg_workspace_build::build_script_output::BuildScriptOutput;

use crate::build_script_environment::BuildScriptEnvironment;
use crate::cfg_atom::cfg_atom;
use crate::configuration::Configuration;
use crate::declared_test_crates::DeclaredTestCrates;
use crate::exact_crate_cfg::ExactCrateCfg;
use crate::feature_graph::FeatureGraph;
use crate::loaded_proc_macros::LoadedProcMacros;
use crate::loaded_project::LoadedProject;
use crate::out_dir_files::OutDirFiles;
use crate::project_model_error::ProjectModelError;
use crate::run_scope::RunScope;
use crate::source_files::SourceFiles;
use crate::supported_target::SupportedTarget;
use crate::target_cfg::TargetCfg;
use crate::target_model::TargetModel;
use crate::workspace_manifest::workspace_manifest;
use crate::workspace_packages::WorkspacePackages;

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
    pub build_outputs: BTreeMap<Configuration, BuildOutputs>,
    pub declared_test_crates: DeclaredTestCrates,
    pub packages: WorkspacePackages,
    pub project: FeatureGraph,
    pub(crate) proc_macro_server: OnceLock<Result<Vec<ProcMacroClient>, Arc<ProjectModelError>>>,
    pub scope: RunScope,
    pub(crate) targets: BTreeMap<SupportedTarget, TargetModel>,
}

impl ResolvedProject {
    /// # Errors
    ///
    /// Returns [`ProjectModelError`] when the cfg tables, the metadata documents or the
    /// rust-analyzer workspace of `configuration` cannot be built.
    pub fn load(
        &self,
        configuration: &Configuration,
    ) -> Result<LoadedProject<'_>, ProjectModelError> {
        let TargetModel {
            cfg:
                TargetCfg {
                    crates,
                    data_layout,
                    library,
                    platform,
                },
            sysroot,
        } = &self.targets[&configuration.target];

        self.cargo_workspace(configuration, platform)
            .map(|cargo| {
                Self::project_workspace(cargo, crates.clone(), data_layout.clone(), sysroot.clone())
            })
            .and_then(|workspace| self.loaded(&workspace, library, configuration))
    }

    fn cargo_workspace(
        &self,
        Configuration { features, .. }: &Configuration,
        platform: &Arc<Platform>,
    ) -> Result<CargoWorkspace, ProjectModelError> {
        self.project
            .build_selected(features, platform)
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
        configuration: &Configuration,
    ) -> Result<LoadedProject<'_>, ProjectModelError> {
        let BuildOutputs {
            build_scripts,
            proc_macro_dylibs,
        } = &self.build_outputs[configuration];
        let WorkspacePackages {
            crate_packages,
            member_targets,
            ..
        } = &self.packages;

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
            build_script_atoms(crate_packages, build_scripts)
                .and_then(|build_script_atoms| {
                    LoadedProcMacros {
                        dylibs: proc_macro_dylibs,
                        server: &self.proc_macro_server,
                        vfs: &vfs,
                        workspace,
                    }
                    .apply(&mut database)
                    .map(|()| build_script_atoms)
                })
                .and_then(|build_script_atoms| {
                    ExactCrateCfg {
                        build_script_atoms: &build_script_atoms,
                        library_atoms,
                        member_targets,
                        vfs: &vfs,
                    }
                    .assign(&mut database);

                    BuildScriptEnvironment {
                        crate_packages,
                        outputs: build_scripts,
                        vfs: &vfs,
                    }
                    .assign(&mut database);

                    OutDirFiles {
                        crate_packages,
                        outputs: build_scripts,
                    }
                    .assign(&mut database, &mut vfs)
                })
                .and_then(|build_script_output_files| {
                    SourceFiles::try_from(&vfs).map(|files| LoadedProject {
                        build_script_output_files,
                        database,
                        files,
                        packages: &self.packages,
                        workspace_root: &self.project.resolved.workspace.root,
                    })
                })
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
}

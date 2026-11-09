use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::OnceLock;

use cargo_metadata::camino::Utf8PathBuf;
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
use ra_ap_proc_macro_api::ProcMacroClient;
use ra_ap_project_model::CargoWorkspace;
use ra_ap_project_model::CfgOverrides;
use ra_ap_project_model::ManifestPath;
use ra_ap_project_model::ProjectWorkspace;
use ra_ap_project_model::ProjectWorkspaceKind;
use ra_ap_project_model::Sysroot;
use ra_ap_project_model::WorkspaceBuildScripts;

use heisenberg_task_graph::once_map::OnceMap;
use heisenberg_workspace_build::build_outputs::BuildOutputs;
use heisenberg_workspace_build::build_script_output::BuildScriptOutput;
use heisenberg_workspace_build::configured_context::ConfiguredContext;

use crate::configuration::Configuration;
use crate::configuration_builds::ConfigurationBuilds;
use crate::declared_test_crates::DeclaredTestCrates;
use crate::feature_graph::FeatureGraph;
use crate::loaded_workspace::LoadedWorkspace;
use crate::out_dir_file::OutDirFile;
use crate::project_model_error::ProjectModelError;
use crate::run_context::RunContext;
use crate::run_scope::RunScope;
use crate::supported_target::SupportedTarget;
use crate::target_cfg::TargetCfg;
use crate::target_model::TargetModel;
use crate::workspace_packages::WorkspacePackages;

pub struct ResolvedProject<'model> {
    pub(crate) build_script_contents:
        OnceMap<Utf8PathBuf, Result<Arc<Vec<OutDirFile>>, Arc<ProjectModelError>>>,
    pub(crate) cargo_environment: Env,
    pub configurations: BTreeMap<Configuration, BTreeMap<String, BuildScriptOutput>>,
    pub declared_test_crates: DeclaredTestCrates,
    pub(crate) library: FeatureGraph,
    pub(crate) manifest: ManifestPath,
    pub packages: WorkspacePackages,
    pub project: FeatureGraph,
    pub(crate) proc_macro_server: OnceLock<Result<Vec<ProcMacroClient>, Arc<ProjectModelError>>>,
    pub(crate) run: RunContext<'model>,
    pub scope: RunScope,
    pub(crate) source_text_checks: OnceMap<AbsPathBuf, Result<(), Arc<ProjectModelError>>>,
}

impl ResolvedProject<'_> {
    /// # Errors
    ///
    /// Returns [`ProjectModelError`] when the sysroot has no compiler or cargo cannot build the
    /// procedural macros and run the build scripts of a configuration.
    pub fn build_outputs(
        &self,
    ) -> Result<BTreeMap<Configuration, BuildOutputs>, ProjectModelError> {
        ConfigurationBuilds {
            configurations: &self.configurations,
            context: &ConfiguredContext {
                cargo_home: self.run.cargo_home.as_ref(),
                environment: &self.run.environment,
                nightly_features_allowed: false,
                workspace_directory: self.run.workspace_directory.as_ref(),
            },
            jobs: self.run.jobs,
            manifest_path: &self.manifest,
            resolved: &self.project.resolved,
            sysroot: self.run.sysroot,
        }
        .outputs()
    }

    /// # Errors
    ///
    /// Returns [`ProjectModelError`] when the feature graph, the metadata documents or the
    /// rust-analyzer workspace of `configuration` cannot be built, or when rust-analyzer
    /// cannot load it.
    pub fn load(
        &self,
        configuration: &Configuration,
        target: &Arc<TargetModel>,
    ) -> Result<LoadedWorkspace<'_>, ProjectModelError> {
        let TargetCfg {
            crates,
            data_layout,
            platform,
            ..
        } = &target.cfg;

        self.cargo_workspace(configuration, platform)
            .map(|cargo| {
                Self::project_workspace(
                    cargo,
                    crates.clone(),
                    data_layout.clone(),
                    target.sysroot.clone(),
                )
            })
            .and_then(|workspace| {
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
                .map(|(database, vfs, _)| LoadedWorkspace {
                    build_script_contents: &self.build_script_contents,
                    database,
                    jobs: self.run.jobs,
                    packages: &self.packages,
                    proc_macro_server: &self.proc_macro_server,
                    source_text_checks: &self.source_text_checks,
                    target: Arc::clone(target),
                    vfs,
                    workspace,
                    workspace_root: &self.project.resolved.workspace.root,
                })
            })
    }

    /// # Errors
    ///
    /// Returns [`ProjectModelError`] when the standard library of a configured target cannot be
    /// resolved.
    pub fn target_models(
        &self,
    ) -> Result<BTreeMap<Configuration, Arc<TargetModel>>, ProjectModelError> {
        let mut targets: BTreeMap<SupportedTarget, Vec<&Configuration>> = BTreeMap::new();

        for configuration in self.configurations.keys() {
            targets
                .entry(configuration.target)
                .or_default()
                .push(configuration);
        }

        targets
            .into_iter()
            .map(|(target, configurations)| {
                TargetModel::new(target, &self.library, self.run.sysroot).map(|model| {
                    let model = Arc::new(model);

                    configurations
                        .into_iter()
                        .map(move |configuration| (configuration.clone(), Arc::clone(&model)))
                })
            })
            .collect::<Result<Vec<_>, ProjectModelError>>()
            .map(|models| models.into_iter().flatten().collect())
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
            .map(|metadata| {
                CargoWorkspace::new(
                    metadata,
                    self.manifest.clone(),
                    self.cargo_environment.clone(),
                    false,
                )
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

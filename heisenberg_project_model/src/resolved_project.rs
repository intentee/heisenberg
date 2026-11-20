use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::OnceLock;

use cargo_metadata::camino::Utf8PathBuf;
use ra_ap_cfg::CfgAtom;
use ra_ap_cfg::CfgDiff;
use ra_ap_ide_db::FxHashMap;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::Env;
use ra_ap_ide_db::base_db::target::Arch;
use ra_ap_ide_db::base_db::target::TargetData;
use ra_ap_intern::sym;
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
use crate::resolved_build::ResolvedBuild;
use crate::run_context::RunContext;
use crate::run_scope::RunScope;
use crate::source_load::SourceLoad;
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
    pub manifest: ManifestPath,
    pub packages: WorkspacePackages,
    pub project: FeatureGraph,
    pub(crate) proc_macro_server: OnceLock<Result<Vec<ProcMacroClient>, Arc<ProjectModelError>>>,
    pub(crate) run: RunContext<'model>,
    pub scope: RunScope,
    pub(crate) source_text_checks: OnceMap<AbsPathBuf, Result<(), Arc<ProjectModelError>>>,
    pub(crate) target_models:
        OnceMap<SupportedTarget, Result<Arc<TargetModel>, Arc<ProjectModelError>>>,
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
    /// Returns [`ProjectModelError`] when the standard library of the target of
    /// `configuration` cannot be resolved, when the feature graph, the metadata documents or
    /// the rust-analyzer workspace of `configuration` cannot be built, or when rust-analyzer
    /// cannot load it.
    pub fn load(
        &self,
        configuration: &Configuration,
    ) -> Result<LoadedWorkspace<'_>, ProjectModelError> {
        self.target_model(configuration.target)
            .map_err(ProjectModelError::TargetModelUnavailable)
            .and_then(|target| self.loaded(configuration, &target))
    }

    /// # Errors
    ///
    /// Returns [`ProjectModelError`] when the standard library of a configured target cannot be
    /// resolved, the first such failure in target order.
    pub fn resolve_standard_libraries(&self) -> Result<(), ProjectModelError> {
        self.configurations
            .keys()
            .map(|Configuration { target, .. }| *target)
            .collect::<BTreeSet<SupportedTarget>>()
            .into_iter()
            .try_for_each(|target| {
                self.target_model(target)
                    .map(drop)
                    .map_err(ProjectModelError::TargetModelUnavailable)
            })
    }

    fn cargo_workspace(&self, build: &ResolvedBuild) -> Result<CargoWorkspace, ProjectModelError> {
        self.project
            .metadata(build)
            .map_err(ProjectModelError::ProjectMetadata)
            .map(|metadata| {
                CargoWorkspace::new(
                    metadata,
                    self.manifest.clone(),
                    self.cargo_environment.clone(),
                    false,
                )
            })
    }

    fn loaded(
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
        let Configuration { features, .. } = configuration;

        self.project
            .build_selected(features, platform)
            .map_err(Box::new)
            .map_err(ProjectModelError::ProjectFeatureResolution)
            .and_then(|build| {
                self.cargo_workspace(&build)
                    .map(|cargo| {
                        Self::project_workspace(
                            cargo,
                            crates.clone(),
                            data_layout.clone(),
                            target.sysroot.clone(),
                        )
                    })
                    .map(|workspace| {
                        let mut database = RootDatabase::new(None);
                        let vfs = SourceLoad {
                            text_checks: &self.source_text_checks,
                            workspace: &workspace,
                        }
                        .load(&mut database);

                        LoadedWorkspace {
                            build,
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
                        }
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

    fn target_model(
        &self,
        target: SupportedTarget,
    ) -> Result<Arc<TargetModel>, Arc<ProjectModelError>> {
        self.target_models.get_or_init(target, || {
            TargetModel::new(target, &self.library, self.run.sysroot)
                .map(Arc::new)
                .map_err(Arc::new)
        })
    }
}

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::num::NonZeroU16;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::OnceLock;

use cargo_metadata::camino::Utf8PathBuf;
use cargo_platform::Cfg;
use ra_ap_cfg::CfgAtom;
use ra_ap_ide_db::RootDatabase;
use ra_ap_paths::AbsPathBuf;
use ra_ap_proc_macro_api::ProcMacroClient;
use ra_ap_project_model::ProjectWorkspace;
use ra_ap_vfs::Vfs;
use ra_ap_vfs::VfsPath;

use heisenberg_error_context::error_context::error_context;
use heisenberg_task_graph::once_map::OnceMap;
use heisenberg_workspace_build::build_outputs::BuildOutputs;
use heisenberg_workspace_build::build_script_output::BuildScriptOutput;

use crate::build_script_environment::BuildScriptEnvironment;
use crate::cfg_atom::cfg_atom;
use crate::exact_crate_cfg::ExactCrateCfg;
use crate::loaded_proc_macros::LoadedProcMacros;
use crate::loaded_project::LoadedProject;
use crate::out_dir_file::OutDirFile;
use crate::out_dir_files::OutDirFiles;
use crate::project_model_error::ProjectModelError;
use crate::source_files::SourceFiles;
use crate::target_model::TargetModel;
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

pub struct LoadedWorkspace<'resolved> {
    pub(crate) build_script_contents:
        &'resolved OnceMap<Utf8PathBuf, Result<Arc<Vec<OutDirFile>>, Arc<ProjectModelError>>>,
    pub(crate) database: RootDatabase,
    pub(crate) jobs: NonZeroU16,
    pub(crate) packages: &'resolved WorkspacePackages,
    pub(crate) proc_macro_server:
        &'resolved OnceLock<Result<Vec<ProcMacroClient>, Arc<ProjectModelError>>>,
    pub(crate) source_text_checks:
        &'resolved OnceMap<AbsPathBuf, Result<(), Arc<ProjectModelError>>>,
    pub(crate) target: Arc<TargetModel>,
    pub(crate) vfs: Vfs,
    pub(crate) workspace: ProjectWorkspace,
    pub(crate) workspace_root: &'resolved AbsPathBuf,
}

impl<'resolved> LoadedWorkspace<'resolved> {
    /// # Errors
    ///
    /// Returns [`ProjectModelError`] when a build script emits a cfg that does not parse, the
    /// procedural macros cannot be loaded, or the build script output cannot be read.
    pub fn built(
        self,
        BuildOutputs {
            build_scripts,
            proc_macro_dylibs,
        }: &BuildOutputs,
    ) -> Result<LoadedProject<'resolved>, ProjectModelError> {
        let Self {
            build_script_contents,
            mut database,
            jobs,
            packages,
            proc_macro_server,
            source_text_checks,
            target,
            mut vfs,
            workspace,
            workspace_root,
        } = self;
        let WorkspacePackages {
            crate_packages,
            member_targets,
            ..
        } = packages;

        build_script_atoms(crate_packages, build_scripts)
            .and_then(|build_script_atoms| {
                LoadedProcMacros {
                    dylibs: proc_macro_dylibs,
                    processes: jobs,
                    server: proc_macro_server,
                    vfs: &vfs,
                    workspace: &workspace,
                }
                .apply(&mut database)
                .map(|()| build_script_atoms)
            })
            .and_then(|build_script_atoms| {
                ExactCrateCfg {
                    build_script_atoms: &build_script_atoms,
                    library_atoms: &target.cfg.library,
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
                    contents: build_script_contents,
                    crate_packages,
                    outputs: build_scripts,
                }
                .assign(&mut database, &mut vfs)
            })
            .and_then(|build_script_output_files| {
                SourceFiles::new(&vfs, workspace_root, source_text_checks).map(|files| {
                    LoadedProject {
                        build_script_output_files,
                        database,
                        files,
                        packages,
                    }
                })
            })
    }
}

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::num::NonZeroU16;
use std::sync::Arc;
use std::sync::OnceLock;
use std::thread::scope;

use ra_ap_ide_db::ChangeWithProcMacros;
use ra_ap_ide_db::FxHashMap;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::CrateBuilderId;
use ra_ap_ide_db::base_db::CrateGraphBuilder;
use ra_ap_ide_db::base_db::DependencyBuilder;
use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8PathBuf;
use ra_ap_proc_macro_api::ProcMacroClient;
use ra_ap_project_model::ProjectWorkspace;
use ra_ap_vfs::FileExcluded;
use ra_ap_vfs::Vfs;
use ra_ap_vfs::VfsPath;

use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
use heisenberg_analysis_threads::joined::joined;

use crate::proc_macro_loading::ProcMacroLoading;
use crate::project_model_error::ProjectModelError;

pub struct LoadedProcMacros<'project> {
    pub dylibs: &'project BTreeMap<Utf8PathBuf, Utf8PathBuf>,
    pub processes: NonZeroU16,
    pub server: &'project OnceLock<Result<Vec<ProcMacroClient>, Arc<ProjectModelError>>>,
    pub vfs: &'project Vfs,
    pub workspace: &'project ProjectWorkspace,
}

impl LoadedProcMacros<'_> {
    pub fn apply(&self, database: &mut RootDatabase) -> Result<(), ProjectModelError> {
        let (crate_graph, _) = self.workspace.to_crate_graph(
            &mut |path| {
                self.vfs
                    .file_id(&VfsPath::from(path.to_path_buf()))
                    .filter(|(_, excluded)| *excluded == FileExcluded::No)
                    .map(|(file_id, _)| file_id)
            },
            &FxHashMap::default(),
        );
        let dylibs: HashMap<VfsPath, AbsPathBuf> = self
            .dylibs
            .iter()
            .map(|(crate_root, dylib)| {
                (
                    VfsPath::from(AbsPathBuf::assert(crate_root.clone())),
                    AbsPathBuf::assert(dylib.clone()),
                )
            })
            .collect();
        let proc_macro_crates: BTreeSet<CrateBuilderId> = crate_graph
            .iter()
            .flat_map(|krate| crate_graph[krate].basic.dependencies.iter())
            .map(|DependencyBuilder { crate_id, .. }| *crate_id)
            .filter(|krate| crate_graph[*krate].basic.is_proc_macro)
            .collect();

        self.servers(&proc_macro_crates)
            .and_then(|servers| {
                scope(|scope| {
                    let threads = &AnalysisThreads::new(scope);
                    let crate_graph = &crate_graph;
                    let dylibs = &dylibs;
                    let loading: Vec<_> = servers
                        .iter()
                        .flat_map(|server| {
                            proc_macro_crates.iter().map(move |krate| {
                                let root =
                                    self.vfs.file_path(crate_graph[*krate].basic.root_file_id);

                                dylibs
                                    .get(root)
                                    .ok_or(ProjectModelError::ProcMacroLibraryMissing(root.clone()))
                                    .and_then(|dylib| {
                                        ProcMacroLoading {
                                            dylib,
                                            server,
                                            threads,
                                        }
                                        .spawn()
                                    })
                            })
                        })
                        .collect();

                    servers
                        .iter()
                        .flat_map(|_| proc_macro_crates.iter().copied())
                        .zip(joined(loading))
                        .map(|(krate, loaded)| loaded.map(|macros| (krate, Ok(macros))))
                        .collect::<Result<Vec<_>, ProjectModelError>>()
                })
            })
            .map(|proc_macros| {
                let mut change = ChangeWithProcMacros::default();

                change.set_proc_macros(proc_macros.into_iter().collect());
                Self::apply_crate_graph(change, crate_graph, database);
            })
    }

    fn apply_crate_graph(
        mut change: ChangeWithProcMacros,
        crate_graph: CrateGraphBuilder,
        database: &mut RootDatabase,
    ) {
        change.set_crate_graph(crate_graph);
        change.apply(database);
    }

    fn servers(
        &self,
        proc_macro_crates: &BTreeSet<CrateBuilderId>,
    ) -> Result<Vec<ProcMacroClient>, ProjectModelError> {
        proc_macro_crates
            .iter()
            .take(1)
            .map(|_| {
                self.server
                    .get_or_init(|| self.spawned_servers())
                    .clone()
                    .map_err(ProjectModelError::ProcMacroServerUnavailable)
            })
            .collect::<Result<Vec<Vec<ProcMacroClient>>, ProjectModelError>>()
            .map(|servers| servers.into_iter().flatten().collect())
    }

    fn spawned_servers(&self) -> Result<Vec<ProcMacroClient>, Arc<ProjectModelError>> {
        let no_environment: FxHashMap<String, Option<String>> = FxHashMap::default();

        self.workspace
            .find_sysroot_proc_macro_srv()
            .into_iter()
            .collect::<Result<Vec<AbsPathBuf>, _>>()
            .map_err(Into::into)
            .map_err(ProjectModelError::ProcMacroServerMissing)
            .and_then(|servers| {
                servers
                    .into_iter()
                    .map(|server| {
                        ProcMacroClient::spawn(
                            &server,
                            no_environment.iter(),
                            None,
                            usize::from(self.processes.get()),
                        )
                        .map_err(|source| {
                            ProjectModelError::ProcMacroServerSpawn {
                                path: server,
                                source,
                            }
                        })
                    })
                    .collect()
            })
            .map_err(Arc::new)
    }
}

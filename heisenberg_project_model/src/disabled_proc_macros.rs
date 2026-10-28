use ra_ap_ide_db::ChangeWithProcMacros;
use ra_ap_ide_db::FxHashMap;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::ProcMacroLoadingError;
use ra_ap_project_model::ProjectWorkspace;
use ra_ap_vfs::FileExcluded;
use ra_ap_vfs::Vfs;
use ra_ap_vfs::VfsPath;

pub struct DisabledProcMacros<'project> {
    pub vfs: &'project Vfs,
    pub workspace: &'project ProjectWorkspace,
}

impl DisabledProcMacros<'_> {
    pub fn apply(&self, database: &mut RootDatabase) {
        let (crate_graph, _) = self.workspace.to_crate_graph(
            &mut |path| {
                self.vfs
                    .file_id(&VfsPath::from(path.to_path_buf()))
                    .filter(|(_, excluded)| *excluded == FileExcluded::No)
                    .map(|(file_id, _)| file_id)
            },
            &FxHashMap::default(),
        );
        let mut change = ChangeWithProcMacros::default();

        change.set_proc_macros(
            crate_graph
                .iter()
                .map(|krate| (krate, Err(ProcMacroLoadingError::Disabled)))
                .collect(),
        );
        change.set_crate_graph(crate_graph);
        change.apply(database);
    }
}

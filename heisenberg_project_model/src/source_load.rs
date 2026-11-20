use std::slice;
use std::sync::Arc;

use crossbeam_channel::unbounded;
use ra_ap_ide_db::ChangeWithProcMacros;
use ra_ap_ide_db::FxHashMap;
use ra_ap_ide_db::RootDatabase;
use ra_ap_load_cargo::ProjectFolders;
use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;
use ra_ap_project_model::ProjectWorkspace;
use ra_ap_vfs::Change;
use ra_ap_vfs::ChangedFile;
use ra_ap_vfs::FileExcluded;
use ra_ap_vfs::Vfs;
use ra_ap_vfs::VfsPath;
use ra_ap_vfs::loader::Config;
use ra_ap_vfs::loader::Handle;
use ra_ap_vfs::loader::LoadingProgress;
use ra_ap_vfs::loader::Message;
use ra_ap_vfs_notify::NotifyHandle;

use heisenberg_task_graph::once_map::OnceMap;

use crate::checked_source_text::checked_source_text;
use crate::project_model_error::ProjectModelError;

pub struct SourceLoad<'load> {
    pub text_checks: &'load OnceMap<AbsPathBuf, Result<(), Arc<ProjectModelError>>>,
    pub workspace: &'load ProjectWorkspace,
}

impl SourceLoad<'_> {
    pub fn load(&self, database: &mut RootDatabase) -> Vfs {
        let (sender, receiver) = unbounded();
        let mut loader = NotifyHandle::spawn(sender);
        let mut vfs = Vfs::default();
        let (crate_graph, _) = self.workspace.to_crate_graph(
            &mut |path| {
                let contents = loader.load_sync(path);

                self.record(path, contents.as_deref());

                let path = VfsPath::from(path.to_path_buf());

                vfs.set_file_contents(path.clone(), contents);
                vfs.file_id(&path)
                    .filter(|(_, excluded)| *excluded == FileExcluded::No)
                    .map(|(file_id, _)| file_id)
            },
            &FxHashMap::default(),
        );
        let ProjectFolders {
            load,
            source_root_config,
            ..
        } = ProjectFolders::new(slice::from_ref(self.workspace), &[], None);

        loader.set_config(Config {
            load,
            version: 0,
            watch: Vec::new(),
        });

        for message in receiver {
            match message {
                Message::Loaded { files } => {
                    for (path, contents) in files {
                        self.record(&path, contents.as_deref());
                        vfs.set_file_contents(VfsPath::from(path), contents);
                    }
                }
                Message::Progress {
                    n_done: LoadingProgress::Finished,
                    ..
                } => break,
                _ => {}
            }
        }

        let mut change = ChangeWithProcMacros::default();

        for ChangedFile {
            change: kind,
            file_id,
        } in vfs.take_changes().into_values()
        {
            if let Change::Create(contents, _) = kind
                && let Ok(text) = String::from_utf8(contents)
            {
                change.change_file(file_id, Some(text));
            }
        }

        change.set_roots(source_root_config.partition(&vfs));
        change.set_crate_graph(crate_graph);
        database.enable_proc_attr_macros();
        database.apply_change(change);

        vfs
    }

    fn record(&self, path: &AbsPath, contents: Option<&[u8]>) {
        if let Some(bytes) = contents {
            self.text_checks
                .initialize(path.to_path_buf(), || checked_source_text(path, bytes));
        }
    }
}

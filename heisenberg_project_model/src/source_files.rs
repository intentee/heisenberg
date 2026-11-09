use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::str;
use std::sync::Arc;

use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8Path;
use ra_ap_vfs::FileId;
use ra_ap_vfs::Vfs;
use ra_ap_vfs::VfsPath;

use heisenberg_error_context::error_context::error_context;
use heisenberg_task_graph::once_map::OnceMap;

use crate::project_model_error::ProjectModelError;
use crate::source_file::SourceFile;
use crate::workspace_path::WorkspacePath;

fn checked_text(path: &AbsPath) -> Result<(), Arc<ProjectModelError>> {
    fs::read(path)
        .map_err(error_context(
            path.to_path_buf(),
            ProjectModelError::SourceFileUnreadable,
        ))
        .and_then(|bytes| {
            str::from_utf8(&bytes).map(drop).map_err(error_context(
                path.to_path_buf(),
                ProjectModelError::SourceFileNotUtf8,
            ))
        })
        .map_err(Arc::new)
}

pub struct SourceFiles {
    files: HashMap<FileId, SourceFile>,
}

impl SourceFiles {
    /// # Errors
    ///
    /// Returns [`ProjectModelError::VirtualSourceFile`] when rust-analyzer holds a file that
    /// is not on disk, and [`ProjectModelError::SourceFileUnavailable`] when a file it holds
    /// cannot be read or is not UTF-8, which leaves it without text in rust-analyzer.
    pub fn new(
        vfs: &Vfs,
        workspace_root: &AbsPath,
        text_checks: &OnceMap<AbsPathBuf, Result<(), Arc<ProjectModelError>>>,
    ) -> Result<Self, ProjectModelError> {
        let workspace_path = WorkspacePath {
            root: AsRef::<Utf8Path>::as_ref(workspace_root),
        };

        vfs.iter()
            .map(|(file_id, path)| (path, file_id))
            .collect::<BTreeMap<&VfsPath, FileId>>()
            .into_iter()
            .map(|(path, file_id)| {
                path.as_path()
                    .ok_or_else(|| ProjectModelError::VirtualSourceFile(path.clone()))
                    .and_then(|real_path| {
                        text_checks
                            .get_or_init(real_path.to_path_buf(), || checked_text(real_path))
                            .map_err(ProjectModelError::SourceFileUnavailable)
                            .map(|()| {
                                (
                                    file_id,
                                    SourceFile {
                                        path: real_path.to_path_buf(),
                                        relative_path: workspace_path.relative(real_path.as_ref()),
                                    },
                                )
                            })
                    })
            })
            .collect::<Result<HashMap<FileId, SourceFile>, ProjectModelError>>()
            .map(|files| Self { files })
    }

    #[must_use]
    pub fn file(&self, file_id: FileId) -> Option<&SourceFile> {
        self.files.get(&file_id)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use ra_ap_paths::AbsPathBuf;
    use ra_ap_vfs::Vfs;
    use ra_ap_vfs::VfsPath;

    use heisenberg_task_graph::once_map::OnceMap;

    use super::SourceFiles;

    #[test]
    fn virtual_source_file_fails_the_source_files() {
        let mut vfs = Vfs::default();

        vfs.set_file_contents(
            VfsPath::new_virtual_path("/injected.rs".to_owned()),
            Some(Vec::new()),
        );

        assert_eq!(
            SourceFiles::new(
                &vfs,
                &AbsPathBuf::assert_utf8(env::current_dir().expect("the test runs in a directory")),
                &OnceMap::default()
            )
            .err()
            .map(|error| error.to_string()),
            Some("rust-analyzer holds `/injected.rs`, which is not a file on disk".to_owned())
        );
    }
}

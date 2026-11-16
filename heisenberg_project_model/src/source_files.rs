use std::collections::BTreeMap;
use std::collections::BTreeSet;
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

use heisenberg_task_graph::once_map::OnceMap;

use crate::project_model_error::ProjectModelError;
use crate::source_file::SourceFile;
use crate::workspace_path::WorkspacePath;

fn checked_text(path: &AbsPath) -> Result<(), Arc<ProjectModelError>> {
    fs::read(path)
        .map_err(|source| ProjectModelError::SourceFileUnreadable {
            path: path.to_path_buf(),
            source,
        })
        .and_then(|bytes| {
            str::from_utf8(&bytes).map(drop).map_err(|source| {
                ProjectModelError::SourceFileNotUtf8 {
                    path: path.to_path_buf(),
                    source,
                }
            })
        })
        .map_err(Arc::new)
}

fn recorded_text_check(
    text_checks: &OnceMap<AbsPathBuf, Result<(), Arc<ProjectModelError>>>,
    real_path: &AbsPath,
) -> Result<(), Arc<ProjectModelError>> {
    text_checks.get_or_init(real_path.to_path_buf(), text_check(real_path))
}

fn text_check(real_path: &AbsPath) -> impl FnOnce() -> Result<(), Arc<ProjectModelError>> + '_ {
    move || checked_text(real_path)
}

pub struct SourceFiles {
    files: HashMap<FileId, SourceFile>,
}

impl SourceFiles {
    pub(crate) fn new(
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
                        recorded_text_check(text_checks, real_path)
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

    #[must_use]
    pub fn under(&self, directories: &[AbsPathBuf]) -> BTreeSet<String> {
        self.files
            .values()
            .filter(|SourceFile { path, .. }| {
                directories
                    .iter()
                    .any(|directory| path.starts_with(directory))
            })
            .map(|SourceFile { relative_path, .. }| relative_path.clone())
            .collect()
    }

    pub(crate) fn check_texts(
        vfs: &Vfs,
        text_checks: &OnceMap<AbsPathBuf, Result<(), Arc<ProjectModelError>>>,
    ) {
        vfs.iter()
            .filter_map(|(_, path)| path.as_path())
            .for_each(|real_path| {
                text_checks.initialize(real_path.to_path_buf(), text_check(real_path));
            });
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::error::Error;
    use std::fs;
    use std::io;
    use std::io::ErrorKind;
    use std::mem;

    use ra_ap_paths::AbsPathBuf;
    use ra_ap_vfs::Vfs;
    use ra_ap_vfs::VfsPath;

    use heisenberg_task_graph::once_map::OnceMap;

    use super::SourceFiles;
    use crate::project_model_error::ProjectModelError;

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

    #[test]
    fn unreadable_source_file_fails_the_source_files() {
        let directory =
            AbsPathBuf::assert_utf8(env::current_dir().expect("the test runs in a directory"));
        let missing = directory.join("missing.rs");
        let reason = fs::read(&missing).err().map(|error| error.to_string());
        let mut vfs = Vfs::default();

        vfs.set_file_contents(VfsPath::from(missing.clone()), Some(Vec::new()));

        let failure = SourceFiles::new(&vfs, &directory, &OnceMap::default()).err();
        let unreadable = ProjectModelError::SourceFileUnreadable {
            path: missing.clone(),
            source: io::Error::from(ErrorKind::NotFound),
        };

        assert!(matches!(
            &failure,
            Some(ProjectModelError::SourceFileUnavailable(unavailable))
                if mem::discriminant(&**unavailable) == mem::discriminant(&unreadable)
        ));
        assert_eq!(
            failure.map(|error| {
                [
                    error.to_string(),
                    error.source().map_or_else(String::new, ToString::to_string),
                ]
            }),
            reason.map(|reason| {
                [
                    format!(
                        "`{missing}` cannot be read to confirm that rust-analyzer loaded its text"
                    ),
                    reason,
                ]
            })
        );
    }
}

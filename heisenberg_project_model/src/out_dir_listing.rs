use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use cargo_metadata::camino::Utf8PathBuf;
use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;
use walkdir::WalkDir;

use crate::out_dir_file::OutDirFile;
use crate::project_model_error::ProjectModelError;

pub struct OutDirListing<'listing> {
    pub out_dir: &'listing AbsPath,
    pub package: &'listing str,
}

impl OutDirListing<'_> {
    /// # Errors
    ///
    /// Returns [`ProjectModelError`] when the build script output cannot be listed, is not a
    /// directory, names a file that is not UTF-8, or holds a file that cannot be read.
    pub fn contents(&self) -> Result<Vec<OutDirFile>, ProjectModelError> {
        self.files().and_then(|files| {
            files
                .into_iter()
                .map(|file| {
                    fs::read(&file)
                        .map_err(|source| ProjectModelError::UnreadableBuildScriptOutput {
                            path: PathBuf::from(file.clone()),
                            source,
                        })
                        .map(|bytes| {
                            bytes
                                .utf8_chunks()
                                .map(|chunk| chunk.invalid().is_empty().then_some(chunk.valid()))
                                .collect::<Option<String>>()
                                .map(|text| OutDirFile { path: file, text })
                        })
                })
                .collect::<Result<Vec<Option<OutDirFile>>, ProjectModelError>>()
                .map(|contents| contents.into_iter().flatten().collect())
        })
    }

    fn files(&self) -> Result<Vec<AbsPathBuf>, ProjectModelError> {
        let Self { out_dir, package } = *self;
        let depth = AsRef::<Path>::as_ref(out_dir).components().count();
        let unlistable = |source: io::Error| ProjectModelError::UnwalkableBuildScriptOutput {
            out_dir: out_dir.to_path_buf(),
            source,
        };

        fs::metadata(out_dir).map_err(unlistable).and_then(|root| {
            if root.is_dir() {
                WalkDir::new(out_dir)
                    .min_depth(1)
                    .sort_by_file_name()
                    .into_iter()
                    .map(|entry| {
                        entry
                            .map_err(io::Error::from)
                            .map_err(unlistable)
                            .and_then(|entry| {
                                Utf8PathBuf::from_path_buf(
                                    entry.path().components().skip(depth).collect(),
                                )
                                .map(|relative| {
                                    if entry.file_type().is_dir() {
                                        Vec::new()
                                    } else {
                                        vec![out_dir.join(relative)]
                                    }
                                })
                                .map_err(|name| {
                                    ProjectModelError::NonUtf8BuildScriptOutputName {
                                        name,
                                        out_dir: out_dir.to_path_buf(),
                                        package: package.to_owned(),
                                    }
                                })
                            })
                    })
                    .collect::<Result<Vec<Vec<AbsPathBuf>>, ProjectModelError>>()
                    .map(|files| files.into_iter().flatten().collect())
            } else {
                Err(ProjectModelError::BuildScriptOutputNotADirectory {
                    out_dir: out_dir.to_path_buf(),
                })
            }
        })
    }
}

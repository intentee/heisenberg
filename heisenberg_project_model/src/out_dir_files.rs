use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::SourceDatabase;
use ra_ap_ide_db::base_db::SourceRoot;
use ra_ap_ide_db::base_db::SourceRootId;
use ra_ap_ide_db::base_db::all_crates;
use ra_ap_ide_db::base_db::salsa::Durability;
use ra_ap_paths::AbsPathBuf;
use ra_ap_vfs::FileId;
use ra_ap_vfs::Vfs;
use ra_ap_vfs::VfsPath;
use ra_ap_vfs::file_set::FileSet;
use triomphe::Arc;

use heisenberg_error_context::error_context::error_context;
use heisenberg_workspace_build::build_script_output::BuildScriptOutput;

use crate::out_dir_file::OutDirFile;
use crate::project_model_error::ProjectModelError;

fn directory_files(directory: &Path) -> Result<Vec<PathBuf>, ProjectModelError> {
    fs::read_dir(directory)
        .and_then(Iterator::collect::<io::Result<Vec<DirEntry>>>)
        .map_err(error_context(
            directory.to_path_buf(),
            ProjectModelError::UnreadableBuildScriptOutput,
        ))
        .and_then(|entries| {
            entries
                .iter()
                .map(|entry| {
                    entry
                        .file_type()
                        .map_err(error_context(
                            entry.path(),
                            ProjectModelError::UnreadableBuildScriptOutput,
                        ))
                        .and_then(|kind| {
                            if kind.is_dir() {
                                directory_files(&entry.path())
                            } else {
                                Ok(vec![entry.path()])
                            }
                        })
                })
                .collect::<Result<Vec<Vec<PathBuf>>, ProjectModelError>>()
                .map(|files| files.into_iter().flatten().collect())
        })
}

pub struct OutDirFiles<'assignment> {
    pub crate_packages: &'assignment HashMap<VfsPath, String>,
    pub outputs: &'assignment BTreeMap<String, BuildScriptOutput>,
}

impl OutDirFiles<'_> {
    pub fn assign(
        &self,
        database: &mut RootDatabase,
        vfs: &mut Vfs,
    ) -> Result<BTreeSet<FileId>, ProjectModelError> {
        let mut source_roots: BTreeMap<SourceRootId, BTreeSet<String>> = BTreeMap::new();

        for krate in all_crates(database).iter() {
            let root_file = krate.data(database).root_file_id;

            if let Some(package) = self
                .crate_packages
                .get(vfs.file_path(root_file))
                .filter(|package| self.outputs.contains_key(*package))
            {
                source_roots
                    .entry(
                        database
                            .file_source_root(root_file)
                            .source_root_id(database),
                    )
                    .or_default()
                    .insert(package.clone());
            }
        }

        source_roots
            .into_iter()
            .map(|(source_root, packages)| {
                packages
                    .iter()
                    .map(|package| directory_files(self.outputs[package].out_dir.as_std_path()))
                    .collect::<Result<Vec<Vec<PathBuf>>, ProjectModelError>>()
                    .and_then(|files| {
                        files
                            .into_iter()
                            .flatten()
                            .map(|file| {
                                fs::read(&file)
                                    .map_err(error_context(
                                        file.clone(),
                                        ProjectModelError::UnreadableBuildScriptOutput,
                                    ))
                                    .map(|bytes| {
                                        String::from_utf8(bytes)
                                            .ok()
                                            .map(|text| OutDirFile { path: file, text })
                                    })
                            })
                            .collect::<Result<Vec<Option<OutDirFile>>, ProjectModelError>>()
                    })
                    .map(|contents| {
                        Self::extend(database, vfs, source_root, contents.into_iter().flatten())
                    })
            })
            .collect::<Result<Vec<Vec<FileId>>, ProjectModelError>>()
            .map(|files| files.into_iter().flatten().collect())
    }

    fn extend(
        database: &mut RootDatabase,
        vfs: &mut Vfs,
        source_root: SourceRootId,
        contents: impl Iterator<Item = OutDirFile>,
    ) -> Vec<FileId> {
        let existing = database.source_root(source_root).source_root(database);
        let mut file_set = FileSet::default();

        for file in existing.iter() {
            file_set.insert(file, vfs.file_path(file).clone());
        }

        let added: Vec<FileId> = contents
            .filter_map(|OutDirFile { path, text }| {
                let path = VfsPath::from(AbsPathBuf::assert_utf8(path));

                vfs.set_file_contents(path.clone(), Some(text.clone().into_bytes()));
                vfs.file_id(&path).map(|(file_id, _)| {
                    database.set_file_text_with_durability(file_id, &text, Durability::MEDIUM);
                    file_set.insert(file_id, path);

                    file_id
                })
            })
            .collect();

        database.set_source_root_with_durability(
            source_root,
            Arc::new(if existing.is_library {
                SourceRoot::new_library(file_set)
            } else {
                SourceRoot::new_local(file_set)
            }),
            Durability::MEDIUM,
        );

        for file_id in &added {
            database.set_file_source_root_with_durability(
                *file_id,
                source_root,
                Durability::MEDIUM,
            );
        }

        added
    }
}

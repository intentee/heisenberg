use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use cargo_metadata::camino::Utf8PathBuf;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::SourceDatabase;
use ra_ap_ide_db::base_db::SourceRoot;
use ra_ap_ide_db::base_db::SourceRootId;
use ra_ap_ide_db::base_db::all_crates;
use ra_ap_ide_db::base_db::salsa::Durability;
use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8Path;

use ra_ap_vfs::FileId;
use ra_ap_vfs::Vfs;
use ra_ap_vfs::VfsPath;
use ra_ap_vfs::file_set::FileSet;
use triomphe::Arc as SharedSourceRoot;

use heisenberg_error_context::error_context::error_context;
use heisenberg_task_graph::once_map::OnceMap;
use heisenberg_workspace_build::build_script_output::BuildScriptOutput;

use crate::crate_package::CratePackage;
use crate::out_dir_file::OutDirFile;
use crate::project_model_error::ProjectModelError;

fn directory_files(
    out_dir: &AbsPath,
    below: &Utf8Path,
    package: &str,
) -> Result<Vec<AbsPathBuf>, ProjectModelError> {
    let directory = out_dir.join(below);

    fs::read_dir(&directory)
        .and_then(Iterator::collect::<io::Result<Vec<DirEntry>>>)
        .map_err(error_context(
            PathBuf::from(directory),
            ProjectModelError::UnreadableBuildScriptOutput,
        ))
        .and_then(|entries| {
            entries
                .iter()
                .map(|entry| {
                    let name = entry.file_name();

                    name.to_str()
                        .map(|written| below.join(written))
                        .ok_or_else(|| ProjectModelError::NonUtf8BuildScriptOutputName {
                            name: below.as_std_path().join(&name),
                            out_dir: out_dir.to_path_buf(),
                            package: package.to_owned(),
                        })
                        .and_then(|relative| {
                            let path = out_dir.join(&relative);

                            entry
                                .file_type()
                                .map_err(error_context(
                                    PathBuf::from(path.clone()),
                                    ProjectModelError::UnreadableBuildScriptOutput,
                                ))
                                .and_then(|kind| {
                                    if kind.is_dir() {
                                        directory_files(out_dir, &relative, package)
                                    } else {
                                        Ok(vec![path])
                                    }
                                })
                        })
                })
                .collect::<Result<Vec<Vec<AbsPathBuf>>, ProjectModelError>>()
                .map(|files| files.into_iter().flatten().collect())
        })
}

fn out_dir_contents(
    out_dir: &AbsPath,
    package: &str,
) -> Result<Vec<OutDirFile>, ProjectModelError> {
    directory_files(out_dir, &Utf8PathBuf::new(), package).and_then(|files| {
        files
            .into_iter()
            .map(|file| {
                fs::read(&file)
                    .map_err(error_context(
                        PathBuf::from(file.clone()),
                        ProjectModelError::UnreadableBuildScriptOutput,
                    ))
                    .map(|bytes| {
                        String::from_utf8(bytes)
                            .ok()
                            .map(|text| OutDirFile { path: file, text })
                    })
            })
            .collect::<Result<Vec<Option<OutDirFile>>, ProjectModelError>>()
            .map(|contents| contents.into_iter().flatten().collect())
    })
}

pub struct OutDirFiles<'assignment> {
    pub contents:
        &'assignment OnceMap<Utf8PathBuf, Result<Arc<Vec<OutDirFile>>, Arc<ProjectModelError>>>,
    pub crate_packages: &'assignment HashMap<VfsPath, CratePackage>,
    pub outputs: &'assignment BTreeMap<String, BuildScriptOutput>,
    pub text_checks: &'assignment OnceMap<AbsPathBuf, Result<(), Arc<ProjectModelError>>>,
}

impl OutDirFiles<'_> {
    pub fn assign(
        &self,
        database: &mut RootDatabase,
        vfs: &mut Vfs,
    ) -> Result<BTreeSet<FileId>, ProjectModelError> {
        let mut source_roots: BTreeMap<SourceRootId, BTreeMap<&CratePackage, &BuildScriptOutput>> =
            BTreeMap::new();

        for krate in all_crates(database).iter() {
            let root_file = krate.data(database).root_file_id;

            if let Some(package) = self.crate_packages.get(vfs.file_path(root_file))
                && let Some(output) = self.outputs.get(&package.id)
            {
                source_roots
                    .entry(
                        database
                            .file_source_root(root_file)
                            .source_root_id(database),
                    )
                    .or_default()
                    .insert(package, output);
            }
        }

        source_roots
            .into_iter()
            .map(|(source_root, packages)| {
                packages
                    .iter()
                    .map(
                        |(CratePackage { name, .. }, BuildScriptOutput { out_dir, .. })| {
                            self.contents
                                .get_or_init(out_dir.clone(), || {
                                    AbsPathBuf::try_from(out_dir.clone())
                                        .map_err(ProjectModelError::RelativeBuildScriptOutput)
                                        .and_then(|directory| out_dir_contents(&directory, name))
                                        .map(Arc::new)
                                        .map_err(Arc::new)
                                })
                                .map_err(ProjectModelError::BuildScriptOutputUnavailable)
                        },
                    )
                    .collect::<Result<Vec<Arc<Vec<OutDirFile>>>, ProjectModelError>>()
                    .map(|contents| {
                        self.extend(
                            database,
                            vfs,
                            source_root,
                            contents
                                .iter()
                                .flat_map(|files| files.iter().cloned())
                                .collect::<Vec<OutDirFile>>()
                                .into_iter(),
                        )
                    })
            })
            .collect::<Result<Vec<Vec<FileId>>, ProjectModelError>>()
            .map(|files| files.into_iter().flatten().collect())
    }

    fn extend(
        &self,
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
                drop(self.text_checks.get_or_init(path.clone(), || Ok(())));

                let path = VfsPath::from(path);

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
            SharedSourceRoot::new(if existing.is_library {
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

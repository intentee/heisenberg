use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
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

use ra_ap_vfs::FileId;
use ra_ap_vfs::Vfs;
use ra_ap_vfs::VfsPath;
use ra_ap_vfs::file_set::FileSet;
use triomphe::Arc as SharedSourceRoot;
use walkdir::WalkDir;

use heisenberg_task_graph::once_map::OnceMap;
use heisenberg_workspace_build::build_script_output::BuildScriptOutput;

use crate::crate_package::CratePackage;
use crate::out_dir_file::OutDirFile;
use crate::project_model_error::ProjectModelError;

fn directory_files(out_dir: &AbsPath, package: &str) -> Result<Vec<AbsPathBuf>, ProjectModelError> {
    let depth = AsRef::<Path>::as_ref(out_dir).components().count();

    WalkDir::new(out_dir)
        .sort_by_file_name()
        .into_iter()
        .map(|entry| {
            entry
                .map_err(|source| ProjectModelError::UnwalkableBuildScriptOutput {
                    out_dir: out_dir.to_path_buf(),
                    source,
                })
                .and_then(|entry| {
                    let is_directory = entry.file_type().is_dir();

                    if entry.depth() == 0 && !is_directory {
                        Err(ProjectModelError::BuildScriptOutputNotADirectory {
                            out_dir: out_dir.to_path_buf(),
                        })
                    } else {
                        Utf8PathBuf::from_path_buf(entry.path().components().skip(depth).collect())
                            .map(|relative| {
                                if is_directory {
                                    Vec::new()
                                } else {
                                    vec![out_dir.join(relative)]
                                }
                            })
                            .map_err(|name| ProjectModelError::NonUtf8BuildScriptOutputName {
                                name,
                                out_dir: out_dir.to_path_buf(),
                                package: package.to_owned(),
                            })
                    }
                })
        })
        .collect::<Result<Vec<Vec<AbsPathBuf>>, ProjectModelError>>()
        .map(|files| files.into_iter().flatten().collect())
}

fn out_dir_contents(
    out_dir: &AbsPath,
    package: &str,
) -> Result<Vec<OutDirFile>, ProjectModelError> {
    directory_files(out_dir, package).and_then(|files| {
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
                self.text_checks.initialize(path.clone(), || Ok(()));

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

#[cfg(test)]
mod tests {
    use std::env;
    use std::error::Error;
    use std::fs;
    #[cfg(unix)]
    use std::os::unix::fs::symlink as symlink_file;
    #[cfg(windows)]
    use std::os::windows::fs::symlink_file;

    use ra_ap_paths::AbsPathBuf;

    use super::directory_files;
    use super::out_dir_contents;

    fn failure_chain(error: &dyn Error) -> Vec<String> {
        let mut chain = vec![error.to_string()];
        let mut cause = error.source();

        while let Some(source) = cause {
            chain.push(source.to_string());
            cause = source.source();
        }

        chain
    }

    #[test]
    fn missing_out_dir_fails_its_listing() {
        let missing = AbsPathBuf::assert_utf8(
            env::temp_dir().join("heisenberg_project_model_missing_out_dir"),
        );
        let reason = fs::symlink_metadata(&missing)
            .err()
            .map(|error| error.to_string());
        let failure = directory_files(&missing, "generator")
            .err()
            .map(|error| failure_chain(&error))
            .map(|chain| [chain.first().cloned(), chain.last().cloned()]);

        assert_eq!(
            failure,
            reason.map(|reason| {
                [
                    Some(format!(
                        "the build script output in `{missing}` cannot be listed"
                    )),
                    Some(reason),
                ]
            })
        );
    }

    #[test]
    fn out_dir_that_is_a_file_fails_its_listing() {
        let file =
            AbsPathBuf::assert_utf8(env::temp_dir().join("heisenberg_project_model_out_dir_file"));

        fs::write(&file, b"").expect("the out-dir must be a file");

        assert_eq!(
            directory_files(&file, "generator")
                .err()
                .map(|error| failure_chain(&error)),
            Some(vec![format!(
                "the build script output `{file}` is not a directory"
            )])
        );
    }

    #[test]
    fn dangling_link_in_an_out_dir_fails_its_reading() {
        let out_dir = AbsPathBuf::assert_utf8(
            env::temp_dir().join("heisenberg_project_model_dangling_out_dir"),
        );
        let link = out_dir.join("dangling.rs");

        fs::create_dir_all(&out_dir)
            .and_then(|()| fs::remove_dir_all(&out_dir))
            .and_then(|()| fs::create_dir_all(&out_dir))
            .and_then(|()| symlink_file(out_dir.join("absent.rs"), &link))
            .expect("the out-dir must hold a dangling link");

        let reason = fs::read(&link).err().map(|error| error.to_string());
        let failure = out_dir_contents(&out_dir, "generator")
            .err()
            .map(|error| failure_chain(&error));

        assert_eq!(
            failure,
            reason.map(|reason| vec![
                format!("the build script output `{link}` cannot be read"),
                reason
            ])
        );
    }
}

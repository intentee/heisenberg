use std::collections::HashMap;

use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;
use ra_ap_vfs::FileId;
use ra_ap_vfs::Vfs;

use crate::project_model_error::ProjectModelError;

pub struct SourceFiles {
    paths: HashMap<FileId, AbsPathBuf>,
}

impl SourceFiles {
    #[must_use]
    pub fn path(&self, file_id: FileId) -> &AbsPath {
        &self.paths[&file_id]
    }
}

impl TryFrom<&Vfs> for SourceFiles {
    type Error = ProjectModelError;

    fn try_from(vfs: &Vfs) -> Result<Self, ProjectModelError> {
        vfs.iter()
            .map(|(file_id, path)| {
                path.as_path()
                    .map(|real_path| (file_id, real_path.to_path_buf()))
                    .ok_or_else(|| ProjectModelError::VirtualSourceFile(path.clone()))
            })
            .collect::<Result<HashMap<FileId, AbsPathBuf>, ProjectModelError>>()
            .map(|paths| Self { paths })
    }
}

#[cfg(test)]
mod tests {
    use ra_ap_vfs::Vfs;
    use ra_ap_vfs::VfsPath;

    use super::SourceFiles;

    #[test]
    fn virtual_source_file_fails_the_source_files() {
        let mut vfs = Vfs::default();

        vfs.set_file_contents(
            VfsPath::new_virtual_path("/injected.rs".to_owned()),
            Some(Vec::new()),
        );

        assert_eq!(
            SourceFiles::try_from(&vfs)
                .err()
                .map(|error| error.to_string()),
            Some("rust-analyzer holds `/injected.rs`, which is not a file on disk".to_owned())
        );
    }
}

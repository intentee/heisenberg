use std::str;
use std::sync::Arc;

use ra_ap_paths::AbsPath;

use crate::project_model_error::ProjectModelError;

pub fn checked_source_text(path: &AbsPath, bytes: &[u8]) -> Result<(), Arc<ProjectModelError>> {
    str::from_utf8(bytes).map(drop).map_err(|source| {
        Arc::new(ProjectModelError::SourceFileNotUtf8 {
            path: path.to_path_buf(),
            source,
        })
    })
}

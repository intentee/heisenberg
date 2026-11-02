use std::path::PathBuf;

use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8PathBuf;

use crate::project_model_error::ProjectModelError;

pub fn absolute_utf8_path(
    path: PathBuf,
    non_utf8: fn(PathBuf) -> ProjectModelError,
    relative: fn(Utf8PathBuf) -> ProjectModelError,
) -> Result<AbsPathBuf, ProjectModelError> {
    Utf8PathBuf::from_path_buf(path)
        .map_err(non_utf8)
        .and_then(|utf8_path| AbsPathBuf::try_from(utf8_path).map_err(relative))
}

use std::fs;
use std::path::Path;

use crate::heisenberg_check_error::HeisenbergCheckError;

/// # Errors
///
/// Returns [`HeisenbergCheckError::FixtureDirectoryCreate`] or
/// [`HeisenbergCheckError::FixtureReset`] when what `path` holds cannot be removed.
pub fn cleared_path(path: &Path) -> Result<(), HeisenbergCheckError> {
    fs::create_dir_all(path)
        .map_err(|source| HeisenbergCheckError::FixtureDirectoryCreate {
            path: path.to_path_buf(),
            source,
        })
        .and_then(|()| {
            fs::remove_dir_all(path).map_err(|source| HeisenbergCheckError::FixtureReset {
                path: path.to_path_buf(),
                source,
            })
        })
}

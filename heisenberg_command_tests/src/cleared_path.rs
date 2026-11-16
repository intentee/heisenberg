use std::fs;
use std::path::Path;

use crate::failure_in::failure_in;
use crate::heisenberg_check_error::HeisenbergCheckError;

/// # Errors
///
/// Returns [`HeisenbergCheckError::ClearedPathCreate`] when `path` cannot be created, and
/// [`HeisenbergCheckError::ClearedPathRemoval`] when what `path` holds cannot be removed.
pub fn cleared_path(path: &Path) -> Result<(), HeisenbergCheckError> {
    fs::create_dir_all(path)
        .map_err(failure_in(
            path.to_path_buf(),
            HeisenbergCheckError::ClearedPathCreate,
        ))
        .and_then(|()| {
            fs::remove_dir_all(path).map_err(failure_in(
                path.to_path_buf(),
                HeisenbergCheckError::ClearedPathRemoval,
            ))
        })
}

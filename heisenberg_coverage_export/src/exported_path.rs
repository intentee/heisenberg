use ra_ap_paths::AbsPathBuf;

use heisenberg_absolute_path::absolute_path::AbsolutePath;
use heisenberg_absolute_path::absolute_path_error::AbsolutePathError;

pub fn exported_path(value: &str) -> Result<AbsPathBuf, AbsolutePathError> {
    AbsolutePath::parse(value).map(|AbsolutePath { path }| path.normalize())
}

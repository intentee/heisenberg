use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8PathBuf;

use crate::absolute_path_error::AbsolutePathError;

#[derive(Clone, Debug)]
pub struct AbsolutePath {
    pub path: AbsPathBuf,
}

impl AbsolutePath {
    pub fn parse(value: &str) -> Result<Self, AbsolutePathError> {
        AbsPathBuf::try_from(Utf8PathBuf::from(value))
            .map(|path| Self { path })
            .map_err(|relative| AbsolutePathError::RelativePath {
                path: relative.into_string(),
            })
    }
}

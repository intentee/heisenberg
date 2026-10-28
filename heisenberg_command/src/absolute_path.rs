use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8PathBuf;

use crate::heisenberg_command_error::HeisenbergCommandError;

#[derive(Clone, Debug)]
pub struct AbsolutePath {
    pub path: AbsPathBuf,
}

impl AbsolutePath {
    pub fn parse(value: &str) -> Result<Self, HeisenbergCommandError> {
        AbsPathBuf::try_from(Utf8PathBuf::from(value))
            .map(|path| Self { path })
            .map_err(|relative| HeisenbergCommandError::RelativePath {
                path: relative.into_string(),
            })
    }
}

use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;

use crate::constants::CARGO_MANIFEST_FILE;

pub fn workspace_manifest(directory: &AbsPath) -> AbsPathBuf {
    directory.join(CARGO_MANIFEST_FILE)
}

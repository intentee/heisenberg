use ra_ap_paths::AbsPathBuf;
use toml::Value;

pub struct WorkspaceDocument {
    pub default_members: Vec<String>,
    pub members: Vec<String>,
    pub metadata: Value,
    pub root: AbsPathBuf,
    pub target_directory: AbsPathBuf,
}

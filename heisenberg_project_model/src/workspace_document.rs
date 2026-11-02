use ra_ap_paths::AbsPathBuf;

pub struct WorkspaceDocument {
    pub default_members: Vec<String>,
    pub members: Vec<String>,
    pub root: AbsPathBuf,
    pub target_directory: AbsPathBuf,
}

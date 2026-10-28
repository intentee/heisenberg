use serde_json::Value;

pub struct WorkspaceDocument {
    pub default_members: Vec<String>,
    pub members: Vec<String>,
    pub root: Value,
    pub target_directory: Value,
}

use std::path::PathBuf;

use serde_json::Value;

pub struct PackageDocument {
    pub id: String,
    pub json: Value,
    pub manifest_path: PathBuf,
    pub name: String,
}

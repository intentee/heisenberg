use cargo_metadata::Package;
use cargo_util_schemas::manifest::TomlLints;

use crate::dependency_document::DependencyDocument;
use crate::workspace_path::WorkspacePath;

pub struct PackageDocument {
    pub dependencies: Vec<DependencyDocument>,
    pub lints: TomlLints,
    pub metadata: Package,
}

impl PackageDocument {
    pub fn relative_manifest(&self, workspace_path: &WorkspacePath<'_>) -> String {
        workspace_path.relative(&self.metadata.manifest_path)
    }
}

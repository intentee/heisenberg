use cargo_metadata::Package;
use cargo_util_schemas::manifest::TomlLints;

use crate::constants::CARGO_MANIFEST_FILE;
use crate::dependency_document::DependencyDocument;
use crate::lint_table::LintTable;
use crate::lints_source::LintsSource;
use crate::workspace_path::WorkspacePath;

pub struct PackageDocument {
    pub dependencies: Vec<DependencyDocument>,
    pub lints: TomlLints,
    pub lints_source: LintsSource,
    pub metadata: Package,
}

impl PackageDocument {
    pub fn lint_table(&self, workspace_path: &WorkspacePath<'_>) -> LintTable {
        match self.lints_source {
            LintsSource::Package => LintTable::Package {
                manifest: self.relative_manifest(workspace_path),
                name: self.metadata.name.to_string(),
            },
            LintsSource::Workspace => LintTable::Workspace {
                manifest: workspace_path.relative(&workspace_path.root.join(CARGO_MANIFEST_FILE)),
            },
        }
    }

    pub fn relative_manifest(&self, workspace_path: &WorkspacePath<'_>) -> String {
        workspace_path.relative(&self.metadata.manifest_path)
    }
}

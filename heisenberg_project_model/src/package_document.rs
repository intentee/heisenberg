use cargo_metadata::Package;
use cargo_util_schemas::manifest::TomlLints;
use serde_json::Value;

use crate::dependency_document::DependencyDocument;
use crate::lint_table::LintTable;
use crate::lints_source::LintsSource;
use crate::unpinned_dependency::UnpinnedDependency;
use crate::versioned_name::versioned_name;
use crate::workspace_path::WorkspacePath;

pub struct PackageDocument {
    pub dependencies: Vec<DependencyDocument>,
    pub is_member: bool,
    pub lints: TomlLints,
    pub lints_source: LintsSource,
    pub metadata: Package,
    pub metadata_json: Value,
    pub rust_version_source: LintsSource,
    pub unpinned_dependencies: Vec<UnpinnedDependency>,
    pub written_rust_version: Option<String>,
}

impl PackageDocument {
    pub fn is_unpublished(&self) -> bool {
        self.metadata.publish.as_ref().is_some_and(Vec::is_empty)
    }

    pub fn lint_table(&self, workspace_path: &WorkspacePath<'_>) -> LintTable {
        match self.lints_source {
            LintsSource::Package => LintTable::Package {
                manifest: self.relative_manifest(workspace_path),
                name: self.metadata.name.to_string(),
            },
            LintsSource::Workspace => LintTable::Workspace {
                manifest: workspace_path.manifest(),
            },
        }
    }

    pub fn relative_manifest(&self, workspace_path: &WorkspacePath<'_>) -> String {
        workspace_path.relative(&self.metadata.manifest_path)
    }

    pub fn rust_version_table(&self, workspace_path: &WorkspacePath<'_>) -> LintTable {
        match self.rust_version_source {
            LintsSource::Package => LintTable::Package {
                manifest: self.relative_manifest(workspace_path),
                name: self.metadata.name.to_string(),
            },
            LintsSource::Workspace => LintTable::Workspace {
                manifest: workspace_path.manifest(),
            },
        }
    }

    pub fn versioned_name(&self) -> String {
        versioned_name(&self.metadata.name, &self.metadata.version)
    }
}

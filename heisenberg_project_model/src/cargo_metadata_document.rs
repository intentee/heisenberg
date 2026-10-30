use std::collections::BTreeMap;

use cargo::GlobalContext;
use cargo::resolver::Resolve;
use cargo::workspace::Package;
use cargo::workspace::PackageId;
use cargo::workspace::Workspace;
use serde_json::Value;
use serde_json::json;

use crate::cargo_dependency_kind::CargoDependencyKind;
use crate::dependency_document::DependencyDocument;
use crate::node_document::NodeDocument;
use crate::package_document::PackageDocument;
use crate::project_model_error::ProjectModelError;
use crate::workspace_document::WorkspaceDocument;

pub struct CargoMetadataDocument<'document, 'context> {
    pub context: &'document GlobalContext,
    pub packages: &'document BTreeMap<PackageId, &'document Package>,
    pub resolve: &'document Resolve,
    pub workspace: &'document Workspace<'context>,
}

impl CargoMetadataDocument<'_, '_> {
    pub fn nodes(&self) -> Result<Vec<NodeDocument>, ProjectModelError> {
        self.packages
            .keys()
            .map(|package_id| {
                Ok(NodeDocument {
                    dependencies: self.dependencies(*package_id)?,
                    id: package_id.to_spec().to_string(),
                })
            })
            .collect()
    }

    pub fn packages(&self) -> Vec<PackageDocument> {
        self.packages
            .values()
            .map(|package| PackageDocument {
                id: package.package_id().to_spec().to_string(),
                json: json!(package.serialized(
                    self.context.cli_unstable(),
                    self.workspace.unstable_features()
                )),
                name: package.name().to_string(),
            })
            .collect()
    }

    pub fn workspace(&self) -> WorkspaceDocument {
        WorkspaceDocument {
            default_members: self
                .workspace
                .default_members()
                .map(|member| member.package_id().to_spec().to_string())
                .collect(),
            members: self
                .workspace
                .members()
                .map(|member| member.package_id().to_spec().to_string())
                .collect(),
            root: json!(self.workspace.root()),
            target_directory: json!(self.workspace.target_dir().as_path_unlocked()),
        }
    }

    fn dependencies(
        &self,
        package_id: PackageId,
    ) -> Result<Vec<DependencyDocument>, ProjectModelError> {
        self.resolve
            .deps(package_id)
            .filter_map(|(dependency_id, dependencies)| {
                self.packages
                    .get(&dependency_id)
                    .and_then(|dependency| {
                        dependency.targets().iter().find(|target| target.is_lib())
                    })
                    .map(|library| {
                        let mut dep_kinds: Vec<Value> = dependencies
                            .iter()
                            .map(|dependency| {
                                json!({"kind": dependency.kind(), "target": dependency.platform()})
                            })
                            .collect();

                        dep_kinds.sort_by_key(Value::to_string);
                        dep_kinds.dedup();

                        self.resolve
                            .extern_crate_name_and_dep_name(package_id, dependency_id, library)
                            .map(|(extern_crate_name, _)| DependencyDocument {
                                json: json!({
                                    "dep_kinds": dep_kinds,
                                    "name": extern_crate_name.as_str(),
                                    "pkg": dependency_id.to_spec().to_string(),
                                }),
                                kinds: dependencies
                                    .iter()
                                    .map(|dependency| {
                                        CargoDependencyKind {
                                            kind: dependency.kind(),
                                        }
                                        .dependency_kind()
                                    })
                                    .collect(),
                                package_id: dependency_id.to_spec().to_string(),
                            })
                            .map_err(|source| ProjectModelError::ExternCrateName {
                                dependency: dependency_id.to_string(),
                                package: package_id.to_string(),
                                source: source.into(),
                            })
                    })
            })
            .collect()
    }
}

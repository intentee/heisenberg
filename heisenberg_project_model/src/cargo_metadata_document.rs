use std::collections::BTreeMap;

use cargo::GlobalContext;
use cargo::resolver::Resolve;
use cargo::workspace::Package;
use cargo::workspace::PackageId;
use cargo::workspace::Target;
use cargo::workspace::Workspace;
use cargo_util_schemas::manifest::InheritableField;
use cargo_util_schemas::manifest::InheritableLints;
use cargo_util_schemas::manifest::TomlManifest;
use serde_json::from_value;
use serde_json::to_value;
use toml::Table;
use toml::Value;

use crate::absolute_utf8_path::absolute_utf8_path;
use crate::dependency_document::DependencyDocument;
use crate::dependency_kind_document::DependencyKindDocument;
use crate::lints_source::LintsSource;
use crate::manifest_dependencies::ManifestDependencies;
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
    pub fn packages(&self) -> Result<Vec<PackageDocument>, ProjectModelError> {
        let libraries: BTreeMap<PackageId, &Target> = self
            .packages
            .iter()
            .filter_map(|(package_id, package)| {
                package
                    .targets()
                    .iter()
                    .find(|target| target.is_lib())
                    .map(|library| (*package_id, library))
            })
            .collect();

        self.packages
            .iter()
            .map(|(package_id, package)| {
                self.dependencies(*package_id, &libraries)
                    .and_then(|dependencies| {
                        to_value(package.serialized(
                            self.context.cli_unstable(),
                            self.workspace.unstable_features(),
                        ))
                        .and_then(from_value)
                        .and_then(|metadata| {
                            to_value(&metadata).map(|metadata_json| PackageDocument {
                                dependencies,
                                is_member: self.workspace.is_member(package),
                                lints: package
                                    .manifest()
                                    .normalized_toml()
                                    .lints
                                    .as_ref()
                                    .map_or_default(|InheritableLints { lints, .. }| lints.clone()),
                                lints_source: if package
                                    .manifest()
                                    .original_toml()
                                    .and_then(|TomlManifest { lints, .. }| lints.as_ref())
                                    .is_some_and(|InheritableLints { workspace, .. }| *workspace)
                                {
                                    LintsSource::Workspace
                                } else {
                                    LintsSource::Package
                                },
                                metadata,
                                metadata_json,
                                rust_version_source: if package
                                    .manifest()
                                    .original_toml()
                                    .and_then(|TomlManifest { package, .. }| package.as_ref())
                                    .and_then(|package| package.rust_version.as_ref())
                                    .is_some_and(InheritableField::is_inherited)
                                {
                                    LintsSource::Workspace
                                } else {
                                    LintsSource::Package
                                },
                                unpinned_dependencies: package
                                    .manifest()
                                    .original_toml()
                                    .filter(|_| self.workspace.is_member(package))
                                    .map_or_else(Vec::new, |manifest| {
                                        ManifestDependencies { manifest }.unpinned()
                                    }),
                                written_rust_version: package
                                    .rust_version()
                                    .map(ToString::to_string),
                            })
                        })
                        .map_err(|source| {
                            ProjectModelError::PackageMetadata {
                                package: package_id.to_string(),
                                source,
                            }
                        })
                    })
            })
            .collect()
    }

    pub fn workspace(&self) -> Result<WorkspaceDocument, ProjectModelError> {
        absolute_utf8_path(
            self.workspace.root().to_path_buf(),
            ProjectModelError::NonUtf8WorkspaceRoot,
            ProjectModelError::RelativeWorkspaceRoot,
        )
        .and_then(|root| {
            absolute_utf8_path(
                self.workspace.target_dir().into_path_unlocked(),
                ProjectModelError::NonUtf8TargetDirectory,
                ProjectModelError::RelativeTargetDirectory,
            )
            .map(|target_directory| WorkspaceDocument {
                unpinned_dependencies: self
                    .workspace
                    .root_maybe()
                    .original_toml()
                    .map_or_else(Vec::new, |manifest| {
                        ManifestDependencies { manifest }.unpinned()
                    }),
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
                metadata: self
                    .workspace
                    .custom_metadata()
                    .cloned()
                    .unwrap_or_else(|| Value::Table(Table::new())),
                root,
                target_directory,
            })
        })
    }

    fn dependencies(
        &self,
        package_id: PackageId,
        libraries: &BTreeMap<PackageId, &Target>,
    ) -> Result<Vec<DependencyDocument>, ProjectModelError> {
        self.resolve
            .deps(package_id)
            .filter_map(|(dependency_id, dependencies)| {
                libraries.get(&dependency_id).map(|library| {
                    self.resolve
                        .extern_crate_name_and_dep_name(package_id, dependency_id, library)
                        .map(|(extern_crate_name, _)| DependencyDocument {
                            extern_crate_name: extern_crate_name.to_string(),
                            kinds: dependencies
                                .iter()
                                .map(|dependency| DependencyKindDocument {
                                    kind: dependency.kind(),
                                    platform: dependency.platform().cloned(),
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

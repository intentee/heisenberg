use std::collections::BTreeMap;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use cargo::ops::resolve_ws;
use cargo::util::data_structures::HashMap;
use cargo::workspace::Package;
use cargo::workspace::PackageId;
use cargo::workspace::Workspace;
use cargo_util_terminal::Shell;
use ra_ap_paths::AbsPath;

use heisenberg_workspace_build::configured_context::ConfiguredContext;

use crate::cargo_configuration_overrides::CargoConfigurationOverrides;
use crate::cargo_metadata_document::CargoMetadataDocument;
use crate::cargo_workspace_role::CargoWorkspaceRole;
use crate::command_line_feature_scope::CommandLineFeatureScope;
use crate::configured_environment::ConfiguredEnvironment;
use crate::project_model_error::ProjectModelError;
use crate::resolved_packages::ResolvedPackages;
use crate::resolver_version::ResolverVersion;
use crate::workspace_manifest::workspace_manifest;
use crate::workspace_member_targets::WorkspaceMemberTargets;

pub struct CargoPackages<'request> {
    pub cargo_home: &'request Path,
    pub environment: &'request HashMap<String, String>,
    pub role: CargoWorkspaceRole,
    pub workspace_directory: &'request AbsPath,
}

impl CargoPackages<'_> {
    pub fn resolve(&self) -> Result<ResolvedPackages, ProjectModelError> {
        let manifest_path: PathBuf = workspace_manifest(self.workspace_directory).into();
        let context = ConfiguredContext {
            cargo_home: self.cargo_home,
            environment: self.environment,
            nightly_features_allowed: self.role == CargoWorkspaceRole::Library,
            workspace_directory: self.workspace_directory.as_ref(),
        }
        .with_shell(Shell::from_write(Box::new(io::sink())))
        .and_then(|context| context.values().map(|_| ()).map(|()| context))
        .map_err(|source| ProjectModelError::CargoConfiguration {
            manifest_path: manifest_path.clone(),
            source: source.into(),
        })?;

        if self.role == CargoWorkspaceRole::Project {
            CargoConfigurationOverrides {
                context: &context,
                manifest_path: &manifest_path,
            }
            .reject()?;
        }

        let mut workspace = Workspace::new(&manifest_path, &context).map_err(|source| {
            ProjectModelError::CargoWorkspace {
                manifest_path: manifest_path.clone(),
                source: source.into(),
            }
        })?;

        if self.role == CargoWorkspaceRole::Project
            && workspace.root() != AsRef::<Path>::as_ref(self.workspace_directory)
        {
            return Err(ProjectModelError::NotTheWorkspaceRoot {
                workspace_directory: self.workspace_directory.to_path_buf().into(),
                workspace_root: workspace.root().to_path_buf(),
            });
        }

        if let Some(member) = workspace
            .members()
            .find(|member| !member.root().starts_with(workspace.root()))
        {
            return Err(ProjectModelError::MemberOutsideWorkspace {
                member_manifest_path: member.manifest_path().to_path_buf(),
                workspace_root: workspace.root().to_path_buf(),
            });
        }

        workspace.set_resolve_honors_rust_version(Some(false));

        let (package_set, resolve) = resolve_ws(&workspace, false).map_err(|source| {
            ProjectModelError::DependencyResolution {
                manifest_path: manifest_path.clone(),
                source: source.into(),
            }
        })?;
        let mut packages: BTreeMap<PackageId, &Package> = BTreeMap::new();

        for package in package_set.get_many(resolve.iter()).map_err(|source| {
            ProjectModelError::PackageSources {
                manifest_path: manifest_path.clone(),
                source: source.into(),
            }
        })? {
            packages.insert(package.package_id(), package);
        }

        let document = CargoMetadataDocument {
            context: &context,
            packages: &packages,
            resolve: &resolve,
            workspace: &workspace,
        };

        document.workspace().and_then(|workspace_document| {
            document.packages().and_then(|package_documents| {
                WorkspaceMemberTargets {
                    workspace: &workspace,
                }
                .collect()
                .and_then(|member_targets| {
                    ConfiguredEnvironment {
                        context: &context,
                        environment: self.environment,
                    }
                    .variables()
                    .map(|configured_environment| ResolvedPackages {
                        command_line_features: if workspace.allows_new_cli_feature_behavior() {
                            CommandLineFeatureScope::EveryMember
                        } else {
                            CommandLineFeatureScope::RootPackage
                        },
                        configured_environment,
                        member_targets,
                        packages: package_documents,
                        resolver: ResolverVersion {
                            behavior: workspace.resolve_behavior(),
                        }
                        .to_guppy(),
                        workspace: workspace_document,
                    })
                })
            })
        })
    }
}

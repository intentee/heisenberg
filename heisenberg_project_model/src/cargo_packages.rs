use std::collections::BTreeMap;
use std::io;
use std::path::Path;

use cargo::GlobalContext;
use cargo::ops::resolve_ws;
use cargo::workspace::Package;
use cargo::workspace::PackageId;
use cargo::workspace::Workspace;
use cargo_util_terminal::Shell;

use crate::cargo_feature_gate::CargoFeatureGate;
use crate::cargo_metadata_document::CargoMetadataDocument;
use crate::project_model_error::ProjectModelError;
use crate::resolved_packages::ResolvedPackages;
use crate::resolver_version::ResolverVersion;
use crate::workspace_member_targets::WorkspaceMemberTargets;

pub struct CargoPackages<'request> {
    pub cargo_feature_gate: CargoFeatureGate,
    pub cargo_home: &'request Path,
    pub workspace_directory: &'request Path,
}

impl CargoPackages<'_> {
    pub fn resolve(&self) -> Result<ResolvedPackages, ProjectModelError> {
        let manifest_path = self.workspace_directory.join("Cargo.toml");
        let mut context = GlobalContext::new(
            Shell::from_write(Box::new(io::sink())),
            self.workspace_directory.to_path_buf(),
            self.cargo_home.to_path_buf(),
        );

        context
            .configure(0, true, None, true, true, true, &None, &[], &[])
            .map_err(|source| ProjectModelError::CargoConfiguration {
                manifest_path: manifest_path.clone(),
                source: source.into(),
            })?;
        context.nightly_features_allowed =
            matches!(self.cargo_feature_gate, CargoFeatureGate::Nightly);

        let mut workspace = Workspace::new(&manifest_path, &context).map_err(|source| {
            ProjectModelError::CargoWorkspace {
                manifest_path: manifest_path.clone(),
                source: source.into(),
            }
        })?;

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

        Ok(ResolvedPackages {
            member_targets: WorkspaceMemberTargets {
                workspace: &workspace,
            }
            .collect(),
            resolver: ResolverVersion {
                behavior: workspace.resolve_behavior(),
            }
            .to_guppy(),
            nodes: document.nodes()?,
            packages: document.packages(),
            workspace: document.workspace(),
            workspace_root: workspace.root().to_path_buf(),
        })
    }
}

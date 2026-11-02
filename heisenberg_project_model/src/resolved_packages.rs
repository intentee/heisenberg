use guppy::graph::cargo::CargoResolverVersion;
use ra_ap_paths::Utf8Path;
use serde_json::Value;
use serde_json::json;

use crate::build_selection::BuildSelection;
use crate::dependency_document::DependencyDocument;
use crate::member_target::MemberTarget;
use crate::package_document::PackageDocument;
use crate::package_edge::PackageEdge;
use crate::workspace_document::WorkspaceDocument;
use crate::workspace_path::WorkspacePath;

pub struct ResolvedPackages {
    pub member_targets: Vec<MemberTarget>,
    pub packages: Vec<PackageDocument>,
    pub resolver: CargoResolverVersion,
    pub workspace: WorkspaceDocument,
}

impl ResolvedPackages {
    pub fn members(&self) -> impl Iterator<Item = &PackageDocument> {
        self.packages
            .iter()
            .filter(|package| self.workspace.members.contains(&package.metadata.id.repr))
    }

    pub fn metadata(&self, selection: &BuildSelection) -> Value {
        let selected: Vec<&PackageDocument> = self
            .packages
            .iter()
            .filter(|package| selection.includes_package(&package.metadata.id.repr))
            .collect();
        let nodes: Vec<Value> = selected
            .iter()
            .map(|package| {
                let id = &package.metadata.id.repr;
                let deps: Vec<&DependencyDocument> = package
                    .dependencies
                    .iter()
                    .filter(|dependency| {
                        selection.includes_edge(&PackageEdge {
                            from: id.clone(),
                            to: dependency.package_id.clone(),
                        })
                    })
                    .collect();

                json!({
                    "dependencies": deps.iter().map(|dependency| dependency.package_id.as_str()).collect::<Vec<&str>>(),
                    "deps": deps
                        .iter()
                        .map(|dependency| json!({
                            "dep_kinds": dependency
                                .kinds
                                .iter()
                                .map(|kind| json!({"kind": kind.kind, "target": kind.platform}))
                                .collect::<Vec<Value>>(),
                            "name": dependency.extern_crate_name,
                            "pkg": dependency.package_id,
                        }))
                        .collect::<Vec<Value>>(),
                    "features": selection.features(id),
                    "id": id,
                })
            })
            .collect();

        json!({
            "metadata": null,
            "packages": selected.iter().map(|package| &package.metadata).collect::<Vec<_>>(),
            "resolve": {"nodes": nodes, "root": null},
            "target_directory": self.workspace.target_directory.as_str(),
            "version": 1,
            "workspace_default_members": self.workspace.default_members,
            "workspace_members": self.workspace.members,
            "workspace_root": self.workspace.root.as_str(),
        })
    }

    pub fn workspace_path(&self) -> WorkspacePath<'_> {
        WorkspacePath {
            root: AsRef::<Utf8Path>::as_ref(&self.workspace.root),
        }
    }
}

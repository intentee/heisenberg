use std::path::PathBuf;

use guppy::graph::cargo::CargoResolverVersion;
use serde_json::Value;
use serde_json::json;

use crate::build_selection::BuildSelection;
use crate::member_target::MemberTarget;
use crate::node_document::NodeDocument;
use crate::package_document::PackageDocument;
use crate::package_edge::PackageEdge;
use crate::workspace_document::WorkspaceDocument;

pub struct ResolvedPackages {
    pub member_targets: Vec<MemberTarget>,
    pub nodes: Vec<NodeDocument>,
    pub packages: Vec<PackageDocument>,
    pub resolver: CargoResolverVersion,
    pub workspace: WorkspaceDocument,
    pub workspace_root: PathBuf,
}

impl ResolvedPackages {
    pub fn metadata(&self, selection: &BuildSelection) -> Value {
        let nodes: Vec<Value> = self
            .nodes
            .iter()
            .filter(|node| selection.includes_package(&node.id))
            .map(|node| {
                let deps: Vec<&Value> = node
                    .dependencies
                    .iter()
                    .filter(|dependency| {
                        selection.includes_edge(&PackageEdge {
                            from: node.id.clone(),
                            to: dependency.package_id.clone(),
                        })
                    })
                    .map(|dependency| &dependency.json)
                    .collect();

                json!({
                    "dependencies": deps.iter().map(|dependency| dependency["pkg"].clone()).collect::<Vec<Value>>(),
                    "deps": deps,
                    "features": selection.features(&node.id),
                    "id": node.id,
                })
            })
            .collect();

        json!({
            "metadata": null,
            "packages": self
                .packages
                .iter()
                .filter(|package| selection.includes_package(&package.id))
                .map(|package| &package.json)
                .collect::<Vec<&Value>>(),
            "resolve": {"nodes": nodes, "root": null},
            "target_directory": self.workspace.target_directory,
            "version": 1,
            "workspace_default_members": self.workspace.default_members,
            "workspace_members": self.workspace.members,
            "workspace_root": self.workspace.root,
        })
    }
}

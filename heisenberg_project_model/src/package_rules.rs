use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::path::Path;

use petgraph::algo::tarjan_scc;
use petgraph::graphmap::DiGraphMap;
use ra_ap_syntax::Edition;
use ra_ap_syntax::SourceFile;
use ra_ap_syntax::ast::HasModuleItem;

use heisenberg_module_graph::crate_target_kind::CrateTargetKind;

use crate::dependency_kind::DependencyKind;
use crate::member_target::MemberTarget;
use crate::node_document::NodeDocument;
use crate::package_document::PackageDocument;
use crate::package_violation::PackageViolation;
use crate::project_model_error::ProjectModelError;
use crate::resolved_packages::ResolvedPackages;
use crate::workspace_path::WorkspacePath;

pub struct PackageRules<'rules> {
    pub resolved: &'rules ResolvedPackages,
}

impl PackageRules<'_> {
    pub fn violations(&self) -> Result<Vec<PackageViolation>, ProjectModelError> {
        let mut violations = self.dependency_cycles();

        violations.extend(self.tests_without_test_libraries()?);
        violations.sort();

        Ok(violations)
    }

    #[expect(
        clippy::expect_used,
        reason = "cargo resolves dev-dependencies only for workspace members, so every package cycle runs through a member"
    )]
    fn dependency_cycles(&self) -> Vec<PackageViolation> {
        let names = self.names();
        let manifests = self.member_manifests();
        let mut graph: DiGraphMap<&str, ()> = DiGraphMap::new();

        for node in &self.resolved.nodes {
            graph.add_node(&node.id);

            for dependency in &node.dependencies {
                graph.add_edge(&node.id, &dependency.package_id, ());
            }
        }

        tarjan_scc(&graph)
            .into_iter()
            .filter(|component| {
                component.len() > 1 || component.iter().any(|id| graph.contains_edge(id, id))
            })
            .map(|component| {
                let packages: BTreeSet<String> =
                    component.iter().map(|id| names[id].to_owned()).collect();
                PackageViolation::PackageDependencyCycle {
                    manifest: component
                        .iter()
                        .filter(|id| manifests.contains_key(*id))
                        .min_by_key(|id| names[*id])
                        .map(|id| manifests[*id].clone())
                        .expect("a dev-dependency cycle runs through a workspace member"),
                    packages: packages.into_iter().collect(),
                }
            })
            .collect()
    }

    fn is_empty_library(root: &Path) -> Result<bool, ProjectModelError> {
        let text =
            read_to_string(root).map_err(|source| ProjectModelError::LibraryRootUnreadable {
                path: root.to_path_buf(),
                source,
            })?;

        Ok(SourceFile::parse(&text, Edition::CURRENT)
            .tree()
            .items()
            .next()
            .is_none())
    }

    fn member_manifests(&self) -> BTreeMap<&str, String> {
        let workspace_path = WorkspacePath {
            root: &self.resolved.workspace_root,
        };

        self.resolved
            .packages
            .iter()
            .filter(|PackageDocument { id, .. }| self.resolved.workspace.members.contains(id))
            .map(
                |PackageDocument {
                     id, manifest_path, ..
                 }| (id.as_str(), workspace_path.relative(manifest_path)),
            )
            .collect()
    }

    fn names(&self) -> BTreeMap<&str, &str> {
        self.resolved
            .packages
            .iter()
            .map(|package| (package.id.as_str(), package.name.as_str()))
            .collect()
    }

    fn tests_without_test_libraries(&self) -> Result<Vec<PackageViolation>, ProjectModelError> {
        let manifests = self.member_manifests();
        let names = self.names();
        let members: BTreeSet<&str> = self
            .resolved
            .workspace
            .members
            .iter()
            .map(String::as_str)
            .collect();
        let nodes: BTreeMap<&str, &NodeDocument> = self
            .resolved
            .nodes
            .iter()
            .map(|node| (node.id.as_str(), node))
            .collect();
        let mut targets: BTreeMap<&str, Vec<&MemberTarget>> = BTreeMap::new();
        let mut violations = Vec::new();

        for member_target in &self.resolved.member_targets {
            targets
                .entry(member_target.crate_target.package_name.as_str())
                .or_default()
                .push(member_target);
        }

        for member in &members {
            let package_targets = &targets[names[member]];
            let others: Vec<&&MemberTarget> = package_targets
                .iter()
                .filter(|target| target.crate_target.kind != CrateTargetKind::IntegrationTest)
                .collect();

            if others.len() < package_targets.len()
                && others.len() == 1
                && others[0].crate_target.kind == CrateTargetKind::Library
                && Self::is_empty_library(&others[0].root)?
            {
                let dependencies: BTreeSet<&str> = nodes[member]
                    .dependencies
                    .iter()
                    .map(|dependency| dependency.package_id.as_str())
                    .collect();
                let destinations: Vec<String> = dependencies
                    .iter()
                    .filter(|candidate| members.contains(*candidate))
                    .filter(|candidate| {
                        let available: BTreeSet<&str> = nodes[*candidate]
                            .dependencies
                            .iter()
                            .filter(|dependency| {
                                dependency.kinds.contains(&DependencyKind::Normal)
                                    || dependency.kinds.contains(&DependencyKind::Development)
                            })
                            .map(|dependency| dependency.package_id.as_str())
                            .collect();

                        dependencies
                            .iter()
                            .filter(|dependency| dependency != candidate)
                            .all(|dependency| available.contains(dependency))
                    })
                    .map(|candidate| names[candidate].to_owned())
                    .collect();

                if !destinations.is_empty() {
                    violations.push(PackageViolation::TestsOnlyPackageWithoutTestLibraries {
                        destinations,
                        manifest: manifests[member].clone(),
                        package: names[member].to_owned(),
                    });
                }
            }
        }

        Ok(violations)
    }
}

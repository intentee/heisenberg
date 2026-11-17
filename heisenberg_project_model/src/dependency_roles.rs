use std::collections::BTreeMap;
use std::collections::BTreeSet;

use cargo::workspace::dependency::DepKind;

use crate::dependency_consumer::DependencyConsumer;
use crate::dependency_document::DependencyDocument;
use crate::dependency_kind_document::DependencyKindDocument;
use crate::dependency_role::DependencyRole;
use crate::dependency_use::DependencyUse;
use crate::manifest_violation::ManifestViolation;
use crate::package_document::PackageDocument;
use crate::package_violation::PackageViolation;
use crate::resolved_packages::ResolvedPackages;

pub struct DependencyRoles<'roles> {
    pub resolved: &'roles ResolvedPackages,
    pub test_crates: &'roles BTreeSet<String>,
    pub uses: &'roles BTreeMap<String, BTreeSet<DependencyUse>>,
}

impl DependencyRoles<'_> {
    #[must_use]
    pub fn violations(&self) -> Vec<PackageViolation> {
        let workspace_path = self.resolved.workspace_path();
        let no_uses = BTreeSet::new();

        self.resolved
            .members()
            .flat_map(|member| {
                let manifest = member.relative_manifest(&workspace_path);
                let uses = self
                    .uses
                    .get(member.metadata.name.as_str())
                    .unwrap_or(&no_uses);

                member
                    .dependencies
                    .iter()
                    .flat_map(|dependency| {
                        self.misdeclared(
                            self.test_crates.contains(member.metadata.name.as_str()),
                            dependency,
                            uses,
                            &manifest,
                        )
                    })
                    .collect::<Vec<ManifestViolation>>()
            })
            .map(PackageViolation::Manifest)
            .collect()
    }

    fn is_member(&self, DependencyDocument { package_id, .. }: &DependencyDocument) -> bool {
        self.resolved
            .members()
            .any(|PackageDocument { metadata, .. }| metadata.id.repr == *package_id)
    }

    fn misdeclared(
        &self,
        is_test_crate: bool,
        dependency: &DependencyDocument,
        uses: &BTreeSet<DependencyUse>,
        manifest: &str,
    ) -> Vec<ManifestViolation> {
        let DependencyDocument {
            extern_crate_name,
            kinds,
            ..
        } = dependency;
        let consumers: BTreeSet<DependencyConsumer> = uses
            .iter()
            .filter(
                |DependencyUse {
                     extern_crate_name: used,
                     ..
                 }| used == extern_crate_name,
            )
            .map(|DependencyUse { consumer, .. }| *consumer)
            .collect();
        let roles: BTreeSet<DepKind> = kinds
            .iter()
            .map(|DependencyKindDocument { kind, .. }| *kind)
            .collect();
        let is_used_by_development = consumers.contains(&DependencyConsumer::Test)
            || consumers.contains(&DependencyConsumer::Example);
        let unused = |role: DependencyRole| ManifestViolation::DependencyUnused {
            dependency: extern_crate_name.clone(),
            manifest: manifest.to_owned(),
            role,
        };

        roles
            .iter()
            .filter_map(|role| match role {
                DepKind::Build => (!consumers.contains(&DependencyConsumer::BuildScript))
                    .then(|| unused(DependencyRole::Build)),
                DepKind::Development if !is_used_by_development => {
                    Some(unused(DependencyRole::Development))
                }
                DepKind::Development => (consumers.contains(&DependencyConsumer::Test)
                    && !is_test_crate
                    && !self.is_member(dependency))
                .then(|| ManifestViolation::TestLibraryInProductionPackage {
                    dependency: extern_crate_name.clone(),
                    manifest: manifest.to_owned(),
                }),
                DepKind::Normal
                    if consumers.contains(&DependencyConsumer::Production)
                        || (is_test_crate && is_used_by_development) =>
                {
                    None
                }
                DepKind::Normal
                    if is_used_by_development && !roles.contains(&DepKind::Development) =>
                {
                    Some(ManifestViolation::TestDependencyInProduction {
                        dependency: extern_crate_name.clone(),
                        manifest: manifest.to_owned(),
                    })
                }
                DepKind::Normal => Some(unused(DependencyRole::Normal)),
            })
            .collect()
    }
}

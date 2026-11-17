use std::collections::BTreeSet;

use crate::constants::SUPPORTED_RUST_VERSION;
use crate::manifest_violation::ManifestViolation;
use crate::package_violation::PackageViolation;
use crate::resolved_packages::ResolvedPackages;

pub struct RustVersions<'versions> {
    pub resolved: &'versions ResolvedPackages,
}

impl RustVersions<'_> {
    #[must_use]
    pub fn violations(&self) -> Vec<PackageViolation> {
        let workspace_path = self.resolved.workspace_path();

        self.resolved
            .members()
            .filter_map(|member| {
                member
                    .metadata
                    .rust_version
                    .as_ref()
                    .filter(|rust_version| **rust_version < SUPPORTED_RUST_VERSION)
                    .and(member.written_rust_version.as_ref())
                    .map(|rust_version| {
                        PackageViolation::Manifest(ManifestViolation::RustVersionBelowSupported {
                            rust_version: rust_version.clone(),
                            table: member.rust_version_table(&workspace_path),
                        })
                    })
            })
            .collect::<BTreeSet<PackageViolation>>()
            .into_iter()
            .collect()
    }
}

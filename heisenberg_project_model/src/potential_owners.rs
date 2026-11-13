use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_graph_cycles::shortest_paths::ShortestPaths;

use crate::dependency_document::DependencyDocument;
use crate::package_document::PackageDocument;
use crate::potential_owner::PotentialOwner;

pub struct PotentialOwners<'owners, 'packages> {
    pub members: &'owners [&'packages PackageDocument],
    pub packages_by_id: &'owners BTreeMap<&'packages str, &'packages PackageDocument>,
    pub paths: &'owners ShortestPaths<&'packages str>,
}

impl PotentialOwners<'_, '_> {
    pub fn of(
        &self,
        PackageDocument { dependencies, .. }: &PackageDocument,
    ) -> Vec<PotentialOwner> {
        let dependencies: BTreeSet<&str> = dependencies
            .iter()
            .map(|DependencyDocument { package_id, .. }| package_id.as_str())
            .collect();
        let owners_by_name: BTreeMap<&str, PotentialOwner> = self
            .members
            .iter()
            .filter(|PackageDocument { metadata, .. }| {
                dependencies.contains(metadata.id.repr.as_str())
            })
            .map(|PackageDocument { metadata, .. }| {
                (
                    metadata.name.as_str(),
                    self.paths
                        .cycle_through(
                            metadata.id.repr.as_str(),
                            &dependencies
                                .iter()
                                .copied()
                                .filter(|dependency| *dependency != metadata.id.repr)
                                .collect(),
                        )
                        .map(|cycle| {
                            cycle
                                .into_iter()
                                .filter_map(|package| self.packages_by_id.get(package))
                                .map(|package| package.versioned_name())
                                .collect::<Vec<String>>()
                        })
                        .map_or_else(
                            || PotentialOwner::Free {
                                package: metadata.name.to_string(),
                            },
                            |cycle| PotentialOwner::Blocked {
                                cycle,
                                package: metadata.name.to_string(),
                            },
                        ),
                )
            })
            .collect();

        owners_by_name.into_values().collect()
    }
}

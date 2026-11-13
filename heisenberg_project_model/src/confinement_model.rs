use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_graph_cycles::graph_edge::GraphEdge;
use heisenberg_graph_cycles::shortest_paths::ShortestPaths;

use crate::build_link::BuildLink;
use crate::build_unit::BuildUnit;
use crate::confined_package::ConfinedPackage;
use crate::confined_package_reach::ConfinedPackageReach;
use crate::confined_reach::ConfinedReach;
use crate::confinement_member::ConfinementMember;
use crate::confinement_violation::ConfinementViolation;
use crate::declared_names::DeclaredNames;
use crate::package_violation::PackageViolation;
use crate::resolved_build::ResolvedBuild;
use crate::versioned_name::versioned_name;

pub struct ConfinementModel {
    pub confined: BTreeMap<String, ConfinedPackage>,
    pub members: Vec<ConfinementMember>,
    pub workspace_manifest: String,
}

impl ConfinementModel {
    #[must_use]
    pub fn reach(&self, ResolvedBuild { links, .. }: &ResolvedBuild) -> ConfinedPackageReach {
        let units: BTreeSet<&BuildUnit> = links
            .iter()
            .flat_map(|BuildLink { from, to, .. }| [from, to])
            .collect();
        let paths = ShortestPaths::new(
            links
                .iter()
                .filter(
                    |BuildLink {
                         development_only, ..
                     }| !development_only,
                )
                .map(|BuildLink { from, to, .. }| GraphEdge { from, to }),
        );
        let mut reach = ConfinedPackageReach::default();

        self.members.iter().for_each(
            |ConfinementMember {
                 id,
                 manifest,
                 name,
                 side,
             }| {
                let is_own =
                    |BuildUnit {
                         package,
                         side: unit_side,
                         ..
                     }: &BuildUnit| { package == id && unit_side == side };
                let start = units.iter().copied().find(|unit| is_own(unit));
                let development: BTreeSet<&BuildUnit> = links
                    .iter()
                    .filter(
                        |BuildLink {
                             development_only,
                             from,
                             ..
                         }| *development_only && is_own(from),
                    )
                    .map(|BuildLink { to, .. }| to)
                    .collect();

                self.confined
                    .iter()
                    .for_each(|(package, ConfinedPackage { ids, users })| {
                        let confined = ConfinedReach {
                            manifest: manifest.clone(),
                            member: name.clone(),
                            package: package.clone(),
                        };

                        if let Some(path) = start.and_then(|start| {
                            paths.path_to(start, &development, |BuildUnit { package, .. }| {
                                package != id && ids.contains(package)
                            })
                        }) {
                            if users.contains(name) {
                                reach.users.insert(confined);
                            } else {
                                reach.breaches.insert(
                                    confined,
                                    path.into_iter()
                                        .map(|BuildUnit { name, version, .. }| {
                                            versioned_name(name, version)
                                        })
                                        .collect(),
                                );
                            }
                        }
                    });
            },
        );

        reach
    }

    #[must_use]
    pub fn unreached_users<'reach>(
        &self,
        reaches: impl IntoIterator<Item = &'reach ConfinedPackageReach>,
    ) -> Vec<PackageViolation> {
        let reaching: BTreeSet<&ConfinedReach> = reaches
            .into_iter()
            .flat_map(|ConfinedPackageReach { users, .. }| users)
            .collect();

        self.confined
            .iter()
            .filter(|(_, ConfinedPackage { ids, .. })| !ids.is_empty())
            .flat_map(|(package, ConfinedPackage { users, .. })| {
                self.members
                    .iter()
                    .filter(|ConfinementMember { manifest, name, .. }| {
                        users.contains(name)
                            && !reaching.contains(&ConfinedReach {
                                manifest: manifest.clone(),
                                member: name.clone(),
                                package: package.clone(),
                            })
                    })
                    .map(|ConfinementMember { name, .. }| {
                        PackageViolation::Confinement(ConfinementViolation::UserNotReachingIt {
                            manifest: self.workspace_manifest.clone(),
                            package: package.clone(),
                            user: name.clone(),
                        })
                    })
                    .collect::<Vec<PackageViolation>>()
            })
            .collect()
    }

    #[must_use]
    pub fn violations(&self) -> Vec<PackageViolation> {
        let mut violations: BTreeSet<PackageViolation> = BTreeSet::new();

        for (package, ConfinedPackage { ids, users }) in &self.confined {
            let DeclaredNames { distinct, repeated } = DeclaredNames::new(users);

            if ids.is_empty() {
                violations.insert(PackageViolation::Confinement(
                    ConfinementViolation::UnknownPackage {
                        manifest: self.workspace_manifest.clone(),
                        package: package.clone(),
                    },
                ));
            }

            violations.extend(repeated.into_iter().map(|user| {
                PackageViolation::Confinement(ConfinementViolation::UserListedTwice {
                    manifest: self.workspace_manifest.clone(),
                    package: package.clone(),
                    user: user.to_owned(),
                })
            }));
            violations.extend(
                distinct
                    .into_iter()
                    .filter(|user| {
                        !self
                            .members
                            .iter()
                            .any(|ConfinementMember { name, .. }| name == user)
                    })
                    .map(|user| {
                        PackageViolation::Confinement(ConfinementViolation::UserNotAMember {
                            manifest: self.workspace_manifest.clone(),
                            package: package.clone(),
                            user: user.to_owned(),
                        })
                    }),
            );
        }

        violations.into_iter().collect()
    }
}

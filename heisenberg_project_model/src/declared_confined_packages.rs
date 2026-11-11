use std::collections::BTreeMap;

use cargo_metadata::Target;
use cargo_metadata::TargetKind;

use crate::build_side::BuildSide;
use crate::confined_package::ConfinedPackage;
use crate::confinement_member::ConfinementMember;
use crate::confinement_model::ConfinementModel;
use crate::package_document::PackageDocument;
use crate::resolved_packages::ResolvedPackages;

pub struct DeclaredConfinedPackages {
    pub packages: BTreeMap<String, Vec<String>>,
}

impl DeclaredConfinedPackages {
    #[must_use]
    pub fn model(&self, resolved: &ResolvedPackages) -> ConfinementModel {
        let workspace_path = resolved.workspace_path();

        ConfinementModel {
            confined: self
                .packages
                .iter()
                .map(|(package, users)| {
                    (
                        package.clone(),
                        ConfinedPackage {
                            ids: resolved
                                .named(package)
                                .map(|PackageDocument { metadata, .. }| metadata.id.repr.clone())
                                .collect(),
                            users: users.clone(),
                        },
                    )
                })
                .collect(),
            members: resolved
                .members()
                .map(|member| ConfinementMember {
                    id: member.metadata.id.repr.clone(),
                    manifest: member.relative_manifest(&workspace_path),
                    name: member.metadata.name.to_string(),
                    side: if member
                        .metadata
                        .targets
                        .iter()
                        .any(|Target { kind, .. }| kind.contains(&TargetKind::ProcMacro))
                    {
                        BuildSide::Host
                    } else {
                        BuildSide::Target
                    },
                })
                .collect(),
            names: resolved
                .packages
                .iter()
                .map(|package| (package.metadata.id.repr.clone(), package.versioned_name()))
                .collect(),
            workspace_manifest: workspace_path.manifest(),
        }
    }
}

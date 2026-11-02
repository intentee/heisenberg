use cargo::workspace::Workspace;

use heisenberg_module_graph::crate_target::CrateTarget;

use crate::absolute_utf8_path::absolute_utf8_path;
use crate::cargo_target_kind::CargoTargetKind;
use crate::member_target::MemberTarget;
use crate::project_model_error::ProjectModelError;

pub struct WorkspaceMemberTargets<'workspace, 'context> {
    pub workspace: &'workspace Workspace<'context>,
}

impl WorkspaceMemberTargets<'_, '_> {
    pub fn collect(&self) -> Result<Vec<MemberTarget>, ProjectModelError> {
        self.workspace
            .members()
            .flat_map(|member| {
                member.targets().iter().map(|target| {
                    target
                        .src_path()
                        .path()
                        .ok_or(ProjectModelError::MetabuildTarget {
                            package: member.name().to_string(),
                            target: target.description_named(),
                        })
                        .and_then(|root| {
                            absolute_utf8_path(
                                root.to_path_buf(),
                                ProjectModelError::NonUtf8CrateRoot,
                                ProjectModelError::RelativeCrateRoot,
                            )
                        })
                        .map(|root| MemberTarget {
                            crate_target: CrateTarget {
                                package_name: member.name().to_string(),
                                target_description: target.description_named(),
                                kind: CargoTargetKind {
                                    kind: target.kind(),
                                }
                                .crate_target_kind(),
                            },
                            root,
                        })
                })
            })
            .collect()
    }
}

use cargo::workspace::Workspace;
use heisenberg_module_graph::crate_target::CrateTarget;

use crate::member_target::MemberTarget;

pub struct WorkspaceMemberTargets<'workspace, 'context> {
    pub workspace: &'workspace Workspace<'context>,
}

impl WorkspaceMemberTargets<'_, '_> {
    pub fn collect(&self) -> Vec<MemberTarget> {
        self.workspace
            .members()
            .flat_map(|member| {
                member.targets().iter().filter_map(|target| {
                    target.src_path().path().map(|root| MemberTarget {
                        crate_target: CrateTarget {
                            package_name: member.name().to_string(),
                            target_description: target.description_named(),
                        },
                        root: root.to_path_buf(),
                    })
                })
            })
            .collect()
    }
}

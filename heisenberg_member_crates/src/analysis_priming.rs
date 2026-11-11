use std::collections::HashMap;

use ra_ap_hir::Crate;
use ra_ap_hir::CrateDependency;
use ra_ap_ide_db::RootDatabase;

use heisenberg_task_graph::task_graph::TaskGraph;
use heisenberg_task_graph::task_node::TaskNode;

use crate::member_crates_error::MemberCratesError;
use crate::priming_step::PrimingStep;

pub struct AnalysisPriming<'priming> {
    pub database: &'priming RootDatabase,
    pub members: &'priming [Crate],
}

impl AnalysisPriming<'_> {
    pub fn prime(&self) -> Result<(), MemberCratesError> {
        let mut indices = HashMap::new();
        let mut nodes = Vec::new();

        for member in self.members {
            self.node(*member, &mut indices, &mut nodes);
        }

        TaskGraph { nodes: &nodes }
            .run(self.database.clone(), |database, step| step.prime(database))
            .map(drop)
            .map_err(MemberCratesError::PrimingSchedule)
    }

    fn node(
        &self,
        krate: Crate,
        indices: &mut HashMap<Crate, usize>,
        nodes: &mut Vec<TaskNode<PrimingStep>>,
    ) -> usize {
        if let Some(known) = indices.get(&krate) {
            return *known;
        }

        let dependencies = krate
            .dependencies(self.database)
            .into_iter()
            .map(|CrateDependency { krate, .. }| self.node(krate, indices, nodes))
            .collect();
        let name_resolution = nodes.len();

        nodes.push(TaskNode {
            dependencies,
            value: PrimingStep::NameResolution(krate),
        });
        nodes.push(TaskNode {
            dependencies: vec![name_resolution],
            value: PrimingStep::TraitImplementations(krate),
        });
        indices.insert(krate, name_resolution);

        name_resolution
    }
}

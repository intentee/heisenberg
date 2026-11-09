use std::collections::HashMap;

use ra_ap_hir::Crate;
use ra_ap_hir::CrateDependency;
use ra_ap_ide_db::RootDatabase;

use heisenberg_task_graph::task_graph::TaskGraph;
use heisenberg_task_graph::task_node::TaskNode;

use crate::module_dependencies_error::ModuleDependenciesError;

pub struct NameResolutionPriming<'priming> {
    pub database: &'priming RootDatabase,
    pub members: &'priming [Crate],
}

impl NameResolutionPriming<'_> {
    pub fn prime(&self) -> Result<(), ModuleDependenciesError> {
        let mut indices = HashMap::new();
        let mut nodes = Vec::new();

        for member in self.members {
            self.node(*member, &mut indices, &mut nodes);
        }

        TaskGraph { nodes: &nodes }
            .run(self.database.clone(), |database, krate| {
                krate.root_module(database);
            })
            .map(drop)
            .map_err(ModuleDependenciesError::NameResolutionSchedule)
    }

    fn node(
        &self,
        krate: Crate,
        indices: &mut HashMap<Crate, usize>,
        nodes: &mut Vec<TaskNode<Crate>>,
    ) -> usize {
        if let Some(known) = indices.get(&krate) {
            return *known;
        }

        let dependencies = krate
            .dependencies(self.database)
            .into_iter()
            .map(|CrateDependency { krate, .. }| self.node(krate, indices, nodes))
            .collect();
        let index = nodes.len();

        nodes.push(TaskNode {
            dependencies,
            value: krate,
        });
        indices.insert(krate, index);

        index
    }
}

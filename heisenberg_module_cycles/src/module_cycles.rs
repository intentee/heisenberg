use std::collections::BTreeMap;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::module_path::ModulePath;

use crate::lifted_dependency::LiftedDependency;
use crate::module_cycle::ModuleCycle;
use crate::sibling_graph::SiblingGraph;

pub struct ModuleCycles<'graph> {
    pub graph: &'graph CrateModuleGraph,
}

impl ModuleCycles<'_> {
    #[must_use]
    pub fn find(&self) -> Vec<ModuleCycle> {
        let mut sibling_graphs: BTreeMap<ModulePath, SiblingGraph> = BTreeMap::new();

        for dependency in &self.graph.dependencies {
            let lifted = LiftedDependency::of(dependency);

            sibling_graphs
                .entry(lifted.container.clone())
                .or_insert_with(|| SiblingGraph::new(lifted.container.clone()))
                .add(lifted, dependency);
        }

        sibling_graphs
            .values()
            .flat_map(|sibling_graph| sibling_graph.cycles(&self.graph.crate_target))
            .collect()
    }
}

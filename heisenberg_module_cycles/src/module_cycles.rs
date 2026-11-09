use std::collections::BTreeMap;

use heisenberg_module_graph::crate_module_graph::CrateModuleGraph;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_path::ModulePath;

use crate::lifted_dependency::LiftedDependency;
use crate::module_cycle::ModuleCycle;
use crate::non_empty::NonEmpty;
use crate::sibling_edge::SiblingEdge;
use crate::sibling_graph::SiblingGraph;

pub struct ModuleCycles<'graph> {
    pub graph: &'graph CrateModuleGraph,
}

impl ModuleCycles<'_> {
    #[must_use]
    pub fn find(&self) -> Vec<ModuleCycle> {
        let mut edges_by_container: BTreeMap<
            ModulePath,
            BTreeMap<SiblingEdge, NonEmpty<ModuleDependency>>,
        > = BTreeMap::new();

        for dependency in &self.graph.findings.dependencies {
            let LiftedDependency { container, edge } = LiftedDependency::of(dependency);

            NonEmpty::push_into(
                edges_by_container.entry(container).or_default(),
                edge,
                dependency.clone(),
            );
        }

        edges_by_container
            .into_iter()
            .flat_map(|(container, edges)| {
                SiblingGraph { container, edges }.cycles(&self.graph.crate_target)
            })
            .collect()
    }
}

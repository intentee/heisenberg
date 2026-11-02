use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_graph_cycles::cycle_components::CycleComponents;
use heisenberg_graph_cycles::graph_edge::GraphEdge;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_path::ModulePath;

use crate::cycle_witness::CycleWitness;
use crate::module_cycle::ModuleCycle;
use crate::module_unit::ModuleUnit;
use crate::non_empty::NonEmpty;
use crate::sibling_edge::SiblingEdge;

pub struct SiblingGraph {
    pub container: ModulePath,
    pub edges: BTreeMap<SiblingEdge, NonEmpty<ModuleDependency>>,
}

impl SiblingGraph {
    #[must_use]
    pub fn cycles(&self, crate_target: &CrateTarget) -> Vec<ModuleCycle> {
        let components = CycleComponents::new(self.edges.keys().map(|edge| GraphEdge {
            from: &edge.from,
            to: &edge.to,
        }));
        let component_of = components.component_of();
        let mut witnesses: BTreeMap<&BTreeSet<&ModuleUnit>, NonEmpty<CycleWitness>> =
            BTreeMap::new();

        for (edge, dependencies) in &self.edges {
            if let Some(units) = component_of
                .get(&edge.from)
                .filter(|units| units.contains(&edge.to))
            {
                let witness = CycleWitness {
                    dependencies: dependencies.clone(),
                    edge: edge.clone(),
                };

                NonEmpty::push_into(&mut witnesses, *units, witness);
            }
        }

        witnesses
            .into_values()
            .map(|witnesses| ModuleCycle {
                container: self.container.clone(),
                crate_target: crate_target.clone(),
                witnesses,
            })
            .collect()
    }
}

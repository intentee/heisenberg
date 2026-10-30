use std::collections::BTreeMap;
use std::collections::BTreeSet;

use petgraph::algo::tarjan_scc;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;

use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_path::ModulePath;

use crate::cycle_witness::CycleWitness;
use crate::lifted_dependency::LiftedDependency;
use crate::module_cycle::ModuleCycle;
use crate::module_unit::ModuleUnit;

pub struct SiblingGraph {
    pub container: ModulePath,
    pub edges: BTreeMap<LiftedDependency, BTreeSet<ModuleDependency>>,
}

impl SiblingGraph {
    #[must_use]
    pub const fn new(container: ModulePath) -> Self {
        Self {
            container,
            edges: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, lifted: LiftedDependency, dependency: &ModuleDependency) {
        self.edges
            .entry(lifted)
            .or_default()
            .insert(dependency.clone());
    }

    #[must_use]
    pub fn cycles(&self, crate_target: &CrateTarget) -> Vec<ModuleCycle> {
        let mut graph: DiGraph<&ModuleUnit, ()> = DiGraph::new();
        let mut indices: BTreeMap<&ModuleUnit, NodeIndex> = BTreeMap::new();

        for lifted in self.edges.keys() {
            let from = *indices
                .entry(&lifted.from)
                .or_insert_with(|| graph.add_node(&lifted.from));
            let to = *indices
                .entry(&lifted.to)
                .or_insert_with(|| graph.add_node(&lifted.to));

            graph.add_edge(from, to, ());
        }

        let mut cycles: Vec<ModuleCycle> = tarjan_scc(&graph)
            .into_iter()
            .filter(|component| component.len() > 1)
            .map(|component| {
                let members: BTreeSet<&ModuleUnit> = component
                    .iter()
                    .filter_map(|index| graph.node_weight(*index).copied())
                    .collect();

                ModuleCycle {
                    container: self.container.clone(),
                    crate_target: crate_target.clone(),
                    units: members.iter().map(|unit| (*unit).clone()).collect(),
                    witnesses: self
                        .edges
                        .iter()
                        .filter(|(lifted, _)| {
                            members.contains(&lifted.from) && members.contains(&lifted.to)
                        })
                        .map(|(lifted, dependencies)| CycleWitness {
                            dependencies: dependencies.iter().cloned().collect(),
                            from: lifted.from.clone(),
                            to: lifted.to.clone(),
                        })
                        .collect(),
                }
            })
            .collect();

        cycles.sort();

        cycles
    }
}

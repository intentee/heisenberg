use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_module_graph::module_dependency::ModuleDependency;
use heisenberg_module_graph::module_path::ModulePath;
use petgraph::algo::tarjan_scc;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;

use crate::cycle_witness::CycleWitness;
use crate::lifted_dependency::LiftedDependency;
use crate::module_cycle::ModuleCycle;
use crate::module_unit::ModuleUnit;

pub struct SiblingGraph {
    pub container: ModulePath,
    pub witnesses: BTreeMap<LiftedDependency, CycleWitness>,
}

impl SiblingGraph {
    #[must_use]
    pub const fn new(container: ModulePath) -> Self {
        Self {
            container,
            witnesses: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, lifted: LiftedDependency, dependency: &ModuleDependency) {
        let witness = CycleWitness {
            dependency: dependency.clone(),
            from: lifted.from.clone(),
            to: lifted.to.clone(),
        };

        self.witnesses.entry(lifted).or_insert(witness);
    }

    #[must_use]
    pub fn cycles(&self, crate_target: &CrateTarget) -> Vec<ModuleCycle> {
        let mut graph: DiGraph<&ModuleUnit, ()> = DiGraph::new();
        let mut indices: BTreeMap<&ModuleUnit, NodeIndex> = BTreeMap::new();

        for witness in self.witnesses.values() {
            let from = *indices
                .entry(&witness.from)
                .or_insert_with(|| graph.add_node(&witness.from));
            let to = *indices
                .entry(&witness.to)
                .or_insert_with(|| graph.add_node(&witness.to));

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
                        .witnesses
                        .values()
                        .filter(|witness| {
                            members.contains(&witness.from) && members.contains(&witness.to)
                        })
                        .cloned()
                        .collect(),
                }
            })
            .collect();

        cycles.sort();

        cycles
    }
}

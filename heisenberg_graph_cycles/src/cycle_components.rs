use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::hash::Hash;

use petgraph::algo::tarjan_scc;
use petgraph::graphmap::DiGraphMap;

use crate::graph_edge::GraphEdge;

pub struct CycleComponents<TNode> {
    pub components: Vec<BTreeSet<TNode>>,
}

impl<TNode: Copy + Hash + Ord> CycleComponents<TNode> {
    pub fn new(edges: impl IntoIterator<Item = GraphEdge<TNode>>) -> Self {
        let mut graph: DiGraphMap<TNode, ()> = DiGraphMap::new();

        for GraphEdge { from, to } in edges {
            graph.add_edge(from, to, ());
        }

        Self {
            components: tarjan_scc(&graph)
                .into_iter()
                .filter(|component| {
                    component.len() > 1
                        || component
                            .iter()
                            .any(|node| graph.contains_edge(*node, *node))
                })
                .map(|component| component.into_iter().collect())
                .collect(),
        }
    }

    #[must_use]
    pub fn component_of(&self) -> BTreeMap<TNode, &BTreeSet<TNode>> {
        self.components
            .iter()
            .flat_map(|component| component.iter().map(move |node| (*node, component)))
            .collect()
    }
}

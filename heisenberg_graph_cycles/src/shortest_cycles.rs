use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::collections::btree_map::Entry;

use crate::graph_edge::GraphEdge;

fn closed_path<TNode: Copy + Ord>(
    previous: &BTreeMap<TNode, TNode>,
    start: TNode,
    last: TNode,
) -> Vec<TNode> {
    let mut reversed = vec![start, last];
    let mut node = last;

    while node != start {
        node = previous[&node];
        reversed.push(node);
    }

    reversed.reverse();
    reversed
}

pub struct ShortestCycles<TNode> {
    successors: BTreeMap<TNode, BTreeSet<TNode>>,
}

impl<TNode: Copy + Ord> ShortestCycles<TNode> {
    pub fn new(edges: impl IntoIterator<Item = GraphEdge<TNode>>) -> Self {
        let mut successors: BTreeMap<TNode, BTreeSet<TNode>> = BTreeMap::new();

        for GraphEdge { from, to } in edges {
            successors.entry(from).or_default().insert(to);
        }

        Self { successors }
    }

    #[must_use]
    pub fn through(&self, start: TNode, start_successors: &BTreeSet<TNode>) -> Option<Vec<TNode>> {
        let mut previous: BTreeMap<TNode, TNode> = BTreeMap::new();
        let mut pending = VecDeque::from([start]);

        while let Some(node) = pending.pop_front() {
            let added = (node == start).then_some(start_successors);

            for next in self
                .successors
                .get(&node)
                .into_iter()
                .chain(added)
                .flatten()
                .copied()
                .collect::<BTreeSet<TNode>>()
            {
                if next == start {
                    return Some(closed_path(&previous, start, node));
                }

                if let Entry::Vacant(vacancy) = previous.entry(next) {
                    vacancy.insert(node);
                    pending.push_back(next);
                }
            }
        }

        None
    }
}

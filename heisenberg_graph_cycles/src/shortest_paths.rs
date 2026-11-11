use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::collections::btree_map::Entry;
use std::iter;

use crate::graph_edge::GraphEdge;

fn walked_path<TNode: Copy + Ord>(
    previous: &BTreeMap<TNode, TNode>,
    start: TNode,
    last: TNode,
    goal: TNode,
) -> Vec<TNode> {
    let mut path: Vec<TNode> = iter::successors(Some(last), |node| {
        previous.get(node).copied().filter(|_| *node != start)
    })
    .collect();

    path.reverse();
    path.push(goal);
    path
}

pub struct ShortestPaths<TNode> {
    successors: BTreeMap<TNode, BTreeSet<TNode>>,
}

impl<TNode: Copy + Ord> ShortestPaths<TNode> {
    pub fn new(edges: impl IntoIterator<Item = GraphEdge<TNode>>) -> Self {
        let mut successors: BTreeMap<TNode, BTreeSet<TNode>> = BTreeMap::new();

        for GraphEdge { from, to } in edges {
            successors.entry(from).or_default().insert(to);
        }

        Self { successors }
    }

    #[must_use]
    pub fn cycle_through(
        &self,
        start: TNode,
        start_successors: &BTreeSet<TNode>,
    ) -> Option<Vec<TNode>> {
        self.search(start, start_successors, |next| next == start)
    }

    pub fn path_to(
        &self,
        start: TNode,
        start_successors: &BTreeSet<TNode>,
        is_goal: impl Fn(TNode) -> bool,
    ) -> Option<Vec<TNode>> {
        self.search(start, start_successors, is_goal)
    }

    fn search(
        &self,
        start: TNode,
        start_successors: &BTreeSet<TNode>,
        is_goal: impl Fn(TNode) -> bool,
    ) -> Option<Vec<TNode>> {
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
                if is_goal(next) {
                    return Some(walked_path(&previous, start, node, next));
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

use std::cell::Cell;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::PoisonError;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use rayon::Scope;
use rayon::scope;

use crate::task_graph_error::TaskGraphError;
use crate::task_node::TaskNode;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct ReadyTask {
    chain: usize,
    node: Reverse<usize>,
}

struct Execution<'graph, TValue, TOutput, TWork> {
    chains: BTreeMap<usize, usize>,
    dependents: Vec<Vec<usize>>,
    nodes: &'graph [TaskNode<TValue>],
    outputs: &'graph [OnceLock<TOutput>],
    pending: Vec<AtomicUsize>,
    ready: Mutex<BinaryHeap<ReadyTask>>,
    work: &'graph TWork,
}

impl<TValue: Sync, TOutput: Send + Sync, TWork: Sync> Execution<'_, TValue, TOutput, TWork> {
    fn next(&self) -> Option<ReadyTask> {
        self.ready
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .pop()
    }

    fn release<'scope, TState: Clone + Send + 'scope>(
        &'scope self,
        scope: &Scope<'scope>,
        node: usize,
        state: TState,
    ) where
        TWork: Fn(&mut TState, &TValue) -> TOutput,
    {
        self.ready
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(ReadyTask {
                chain: self.chains.get(&node).copied().unwrap_or_default(),
                node: Reverse(node),
            });
        scope.spawn(move |scope| self.run_ready(scope, &state));
    }

    fn run_ready<'scope, TState: Clone + Send + 'scope>(
        &'scope self,
        scope: &Scope<'scope>,
        state: &TState,
    ) where
        TWork: Fn(&mut TState, &TValue) -> TOutput,
    {
        while let Some(ReadyTask {
            node: Reverse(node),
            ..
        }) = self.next()
        {
            self.nodes
                .get(node)
                .zip(self.outputs.get(node))
                .into_iter()
                .for_each(|(TaskNode { value, .. }, output)| {
                    output.get_or_init(|| (self.work)(&mut state.clone(), value));
                });

            for dependent in self.dependents.get(node).into_iter().flatten() {
                if self
                    .pending
                    .get(*dependent)
                    .is_some_and(|pending| pending.fetch_sub(1, Ordering::AcqRel) == 1)
                {
                    self.release(scope, *dependent, state.clone());
                }
            }
        }
    }
}

pub struct TaskGraph<'graph, TValue> {
    pub nodes: &'graph [TaskNode<TValue>],
}

impl<TValue: Sync> TaskGraph<'_, TValue> {
    /// # Errors
    ///
    /// Returns [`TaskGraphError::UnknownDependency`] when a node depends on an index the graph
    /// does not hold, and [`TaskGraphError::DependencyCycle`] when some nodes can never start.
    pub fn run<TState: Clone + Send, TOutput: Send + Sync>(
        &self,
        state: TState,
        work: impl Fn(&mut TState, &TValue) -> TOutput + Sync,
    ) -> Result<Vec<TOutput>, TaskGraphError> {
        let outputs: Vec<OnceLock<TOutput>> = self.nodes.iter().map(|_| OnceLock::new()).collect();

        self.execute(&outputs, state, &work)?;

        let results: Vec<Option<TOutput>> = outputs.into_iter().map(OnceLock::into_inner).collect();
        let blocked: Vec<usize> = results
            .iter()
            .enumerate()
            .filter(|(_, output)| output.is_none())
            .map(|(node, _)| node)
            .collect();

        if blocked.is_empty() {
            Ok(results.into_iter().flatten().collect())
        } else {
            Err(TaskGraphError::DependencyCycle { blocked })
        }
    }

    fn chains(&self, dependents: &[Vec<usize>]) -> BTreeMap<usize, usize> {
        let mut chains: BTreeMap<usize, usize> = BTreeMap::new();
        let unmeasured: Vec<Cell<usize>> = dependents
            .iter()
            .map(|waiting| Cell::new(waiting.len()))
            .collect();
        let mut measurable: Vec<usize> = dependents
            .iter()
            .enumerate()
            .filter(|(_, waiting)| waiting.is_empty())
            .map(|(node, _)| node)
            .collect();

        while let Some(node) = measurable.pop() {
            let chain = dependents
                .get(node)
                .into_iter()
                .flatten()
                .filter_map(|dependent| chains.get(dependent))
                .max()
                .copied()
                .unwrap_or_default()
                .saturating_add(1);

            chains.insert(node, chain);

            for dependency in self
                .nodes
                .get(node)
                .into_iter()
                .flat_map(|TaskNode { dependencies, .. }| dependencies)
            {
                if unmeasured.get(*dependency).is_some_and(|count| {
                    count.set(count.get().saturating_sub(1));
                    count.get() == 0
                }) {
                    measurable.push(*dependency);
                }
            }
        }

        chains
    }

    fn dependents(&self) -> Result<Vec<Vec<usize>>, TaskGraphError> {
        let mut dependents: Vec<Vec<usize>> = self.nodes.iter().map(|_| Vec::new()).collect();

        for (node, TaskNode { dependencies, .. }) in self.nodes.iter().enumerate() {
            for dependency in dependencies {
                dependents
                    .get_mut(*dependency)
                    .ok_or(TaskGraphError::UnknownDependency {
                        dependency: *dependency,
                        node,
                    })?
                    .push(node);
            }
        }

        Ok(dependents)
    }

    fn execute<TState: Clone + Send, TOutput: Send + Sync>(
        &self,
        outputs: &[OnceLock<TOutput>],
        state: TState,
        work: &(impl Fn(&mut TState, &TValue) -> TOutput + Sync),
    ) -> Result<(), TaskGraphError> {
        self.dependents().map(|dependents| {
            let execution = Execution {
                chains: self.chains(&dependents),
                dependents,
                nodes: self.nodes,
                outputs,
                pending: self
                    .nodes
                    .iter()
                    .map(|TaskNode { dependencies, .. }| AtomicUsize::new(dependencies.len()))
                    .collect(),
                ready: Mutex::default(),
                work,
            };
            let execution = &execution;

            scope(move |scope| {
                for (node, TaskNode { dependencies, .. }) in self.nodes.iter().enumerate() {
                    if dependencies.is_empty() {
                        execution.release(scope, node, state.clone());
                    }
                }
            });
        })
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    use std::panic::AssertUnwindSafe;
    use std::sync::Barrier;
    use std::sync::Mutex;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use rayon::ThreadPool;
    use rayon::ThreadPoolBuilder;
    use rayon::current_thread_index;

    use super::TaskGraph;
    use crate::task_graph_error::TaskGraphError;
    use crate::task_node::TaskNode;

    fn pool(workers: usize) -> ThreadPool {
        ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .expect("the test pool must start")
    }

    fn diamond_lattice() -> Vec<TaskNode<usize>> {
        (0..64)
            .map(|node: usize| TaskNode {
                dependencies: (1..node)
                    .filter(|candidate| node.is_multiple_of(*candidate))
                    .collect(),
                value: node,
            })
            .collect()
    }

    fn node_value() -> impl Fn(&mut (), &usize) -> usize + Sync {
        |(), node| *node
    }

    #[test]
    fn every_task_runs_exactly_once() {
        let nodes = diamond_lattice();
        let runs: Vec<AtomicUsize> = nodes.iter().map(|_| AtomicUsize::new(0)).collect();

        pool(4).install(|| {
            TaskGraph { nodes: &nodes }
                .run((), |(), node| {
                    runs.get(*node)
                        .expect("every node has a counter")
                        .fetch_add(1, Ordering::SeqCst);
                })
                .expect("the lattice is acyclic")
        });

        assert!(runs.iter().all(|count| count.load(Ordering::SeqCst) == 1));
    }

    #[test]
    fn a_task_starts_only_after_its_dependencies_finished() {
        let nodes = diamond_lattice();
        let finished: Vec<AtomicBool> = nodes.iter().map(|_| AtomicBool::new(false)).collect();
        let started_after_dependencies = pool(4).install(|| {
            TaskGraph { nodes: &nodes }
                .run((), |(), node| {
                    let ready = nodes
                        .get(*node)
                        .expect("every node exists")
                        .dependencies
                        .iter()
                        .all(|dependency| {
                            finished
                                .get(*dependency)
                                .expect("every dependency exists")
                                .load(Ordering::SeqCst)
                        });

                    finished
                        .get(*node)
                        .expect("every node has a flag")
                        .store(true, Ordering::SeqCst);

                    ready
                })
                .expect("the lattice is acyclic")
        });

        assert_eq!(started_after_dependencies, vec![true; nodes.len()]);
    }

    #[test]
    fn tasks_run_only_on_the_workers_of_the_pool() {
        let nodes = diamond_lattice();
        let workers = pool(3).install(|| {
            TaskGraph { nodes: &nodes }
                .run((), |(), _| current_thread_index())
                .expect("the lattice is acyclic")
        });

        assert_eq!(workers.len(), nodes.len());
        assert!(
            workers
                .iter()
                .all(|worker| worker.is_some_and(|index| index < 3))
        );
    }

    #[test]
    fn independent_tasks_run_concurrently() {
        let nodes: Vec<TaskNode<usize>> = (0..4)
            .map(|node| TaskNode {
                dependencies: Vec::new(),
                value: node,
            })
            .collect();
        let all_running = Barrier::new(4);
        let leaders = pool(4).install(|| {
            TaskGraph { nodes: &nodes }
                .run((), |(), _| all_running.wait().is_leader())
                .expect("independent tasks are acyclic")
        });

        assert_eq!(leaders.iter().filter(|leader| **leader).count(), 1);
    }

    #[test]
    fn ready_tasks_start_in_order_of_the_longest_chain_waiting_on_them() {
        let nodes = vec![
            TaskNode {
                dependencies: Vec::new(),
                value: 0,
            },
            TaskNode {
                dependencies: Vec::new(),
                value: 1,
            },
            TaskNode {
                dependencies: vec![0],
                value: 2,
            },
            TaskNode {
                dependencies: vec![2],
                value: 3,
            },
        ];
        let started = Mutex::new(Vec::new());

        pool(1).install(|| {
            TaskGraph { nodes: &nodes }
                .run((), |(), node| {
                    started
                        .lock()
                        .expect("no task panics while it holds the record")
                        .push(*node);
                })
                .expect("the graph is acyclic")
        });

        assert_eq!(
            started
                .into_inner()
                .expect("no task panics while it holds the record"),
            vec![0, 2, 1, 3]
        );
    }

    #[test]
    fn results_follow_node_order_for_every_worker_count() {
        let nodes = diamond_lattice();
        let results: Vec<Vec<usize>> = (1..=8)
            .map(|workers| {
                pool(workers).install(|| {
                    TaskGraph { nodes: &nodes }
                        .run((), |(), node| node.pow(2))
                        .expect("the lattice is acyclic")
                })
            })
            .collect();

        assert!(results.iter().all(|outputs| {
            *outputs
                == (0..64)
                    .map(|node: usize| node.pow(2))
                    .collect::<Vec<usize>>()
        }));
    }

    #[test]
    fn a_single_worker_completes_a_deep_chain() {
        let nodes: Vec<TaskNode<usize>> = (0..10_000)
            .map(|node: usize| TaskNode {
                dependencies: node.checked_sub(1).into_iter().collect(),
                value: node,
            })
            .collect();

        assert_eq!(
            pool(1)
                .install(|| TaskGraph { nodes: &nodes }.run((), node_value()))
                .map(|outputs| outputs.len()),
            Ok(10_000)
        );
    }

    #[test]
    fn every_task_receives_its_own_copy_of_the_state() {
        let nodes = diamond_lattice();
        let outputs = pool(4).install(|| {
            TaskGraph { nodes: &nodes }
                .run(Vec::<usize>::new(), |seen, node| {
                    seen.push(*node);

                    seen.len()
                })
                .expect("the lattice is acyclic")
        });

        assert_eq!(outputs, vec![1; nodes.len()]);
    }

    #[test]
    fn a_panicking_task_reaches_the_caller() {
        let nodes = diamond_lattice();
        let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
            pool(4).install(|| {
                TaskGraph { nodes: &nodes }.run((), |(), node| {
                    assert_ne!(*node, 7, "task 7 fails");
                })
            })
        }));

        assert!(outcome.is_err());
    }

    #[test]
    fn a_dependency_outside_the_graph_is_rejected() {
        let nodes = vec![TaskNode {
            dependencies: vec![1],
            value: 0,
        }];

        assert_eq!(
            pool(1).install(|| TaskGraph { nodes: &nodes }.run((), node_value())),
            Err(TaskGraphError::UnknownDependency {
                dependency: 1,
                node: 0,
            })
        );
    }

    #[test]
    fn tasks_in_or_behind_a_cycle_are_reported() {
        let nodes = vec![
            TaskNode {
                dependencies: Vec::new(),
                value: 0,
            },
            TaskNode {
                dependencies: vec![2],
                value: 1,
            },
            TaskNode {
                dependencies: vec![1],
                value: 2,
            },
            TaskNode {
                dependencies: vec![2],
                value: 3,
            },
        ];

        assert_eq!(
            pool(2).install(|| TaskGraph { nodes: &nodes }.run((), node_value())),
            Err(TaskGraphError::DependencyCycle {
                blocked: vec![1, 2, 3],
            })
        );
    }
}

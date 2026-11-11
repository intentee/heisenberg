use std::collections::BTreeMap;
use std::sync::OnceLock;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use rayon::scope;

use crate::task::Task;
use crate::task_graph_error::TaskGraphError;
use crate::task_node::TaskNode;

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

    fn execute<TState: Clone + Send, TOutput: Send + Sync>(
        &self,
        outputs: &[OnceLock<TOutput>],
        state: TState,
        work: &(impl Fn(&mut TState, &TValue) -> TOutput + Sync),
    ) -> Result<(), TaskGraphError> {
        let tasks: Vec<Task<'_, TValue, TOutput>> = self
            .nodes
            .iter()
            .zip(outputs)
            .map(
                |(
                    TaskNode {
                        dependencies,
                        value,
                    },
                    output,
                )| Task {
                    dependents: OnceLock::new(),
                    output,
                    pending: AtomicUsize::new(dependencies.len()),
                    value,
                },
            )
            .collect();

        let tasks = &tasks;

        self.link(tasks).map(|()| {
            scope(move |scope| {
                for task in tasks
                    .iter()
                    .filter(|Task { pending, .. }| pending.load(Ordering::Acquire) == 0)
                {
                    task.spawn(scope, state.clone(), work);
                }
            });
        })
    }

    fn link<'tasks, TOutput>(
        &self,
        tasks: &'tasks [Task<'tasks, TValue, TOutput>],
    ) -> Result<(), TaskGraphError> {
        let mut dependents: BTreeMap<usize, Vec<&'tasks Task<'tasks, TValue, TOutput>>> =
            BTreeMap::new();

        for (node, (TaskNode { dependencies, .. }, task)) in
            self.nodes.iter().zip(tasks).enumerate()
        {
            for dependency in dependencies {
                tasks
                    .get(*dependency)
                    .ok_or(TaskGraphError::UnknownDependency {
                        dependency: *dependency,
                        node,
                    })?;
                dependents.entry(*dependency).or_default().push(task);
            }
        }

        for (node, task) in tasks.iter().enumerate() {
            task.dependents
                .get_or_init(|| dependents.remove(&node).unwrap_or_default());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    use std::panic::AssertUnwindSafe;
    use std::sync::Barrier;
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

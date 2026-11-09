use thiserror::Error;

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TaskGraphError {
    #[error("tasks {} depend on each other in a cycle, so they can never start", blocked.iter().map(ToString::to_string).collect::<Vec<String>>().join(", "))]
    DependencyCycle { blocked: Vec<usize> },
    #[error("task {node} depends on task {dependency}, which the graph does not hold")]
    UnknownDependency { dependency: usize, node: usize },
}

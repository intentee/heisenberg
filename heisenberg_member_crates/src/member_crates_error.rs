use thiserror::Error;

use heisenberg_source_locator::source_locator_error::SourceLocatorError;
use heisenberg_task_graph::task_graph_error::TaskGraphError;

#[derive(Debug, Error)]
pub enum MemberCratesError {
    #[error(
        "name resolution and trait implementations cannot be scheduled over the crate dependency graph"
    )]
    PrimingSchedule(#[source] TaskGraphError),
    #[error(transparent)]
    SourceLocator(#[from] SourceLocatorError),
}

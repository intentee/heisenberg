use std::io;

use thiserror::Error;

use heisenberg_analysis_threads::analysis_threads_error::AnalysisThreadsError;
use heisenberg_analysis_threads::panic_payload::PanicPayload;
use heisenberg_error_context::contextual_error::ContextualError;
use heisenberg_error_context::error_chain::error_chain;

#[derive(Debug, Error)]
pub enum ModuleDependenciesError {
    #[error("the host does not report its available parallelism")]
    AvailableParallelism {
        #[from]
        source: io::Error,
    },
    #[error(
        "name resolution priming fails: {}",
        .0.iter().flat_map(error_chain).collect::<Vec<String>>().join("; ")
    )]
    NameResolution(Vec<Self>),
    #[error("the name resolution thread for {} panicked", .0.context.join(", "))]
    NameResolutionPanicked(#[source] ContextualError<Vec<String>, PanicPayload>),
    #[error("the name resolution thread for {} cannot be started", .0.context.join(", "))]
    NameResolutionThread(#[source] ContextualError<Vec<String>, AnalysisThreadsError>),
}

use std::io;
use std::num::NonZeroU16;
use std::sync::Arc;

use rayon::ThreadPoolBuildError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalysisThreadsError {
    #[error("the operating system cannot start a thread")]
    Spawn {
        #[from]
        source: io::Error,
    },
    #[error(
        "the worker pool of this process runs {configured} workers, so it cannot run {requested}"
    )]
    WorkerCountChanged {
        configured: NonZeroU16,
        requested: NonZeroU16,
    },
    #[error("the worker pool of {workers} workers cannot be started")]
    WorkerPoolBuild {
        #[source]
        source: Arc<ThreadPoolBuildError>,
        workers: NonZeroU16,
    },
}

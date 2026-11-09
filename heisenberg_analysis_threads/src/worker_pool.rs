use std::num::NonZeroU16;
use std::sync::Arc;
use std::sync::OnceLock;

use ra_ap_stdx::thread::DEFAULT_STACK_SIZE;
use rayon::ThreadPoolBuildError;
use rayon::ThreadPoolBuilder;
use rayon::spawn;

use crate::analysis_threads_error::AnalysisThreadsError;

static PROCESS_POOL: OnceLock<WorkerPool> = OnceLock::new();

pub struct WorkerPool {
    started: Result<(), Arc<ThreadPoolBuildError>>,
    workers: NonZeroU16,
}

impl WorkerPool {
    pub fn release<TValue: Send + 'static>(value: TValue) {
        spawn(move || drop(value));
    }

    /// # Errors
    ///
    /// Returns [`AnalysisThreadsError::WorkerPoolBuild`] when the worker pool of the process
    /// cannot start, and [`AnalysisThreadsError::WorkerCountChanged`] when it already runs
    /// another number of workers.
    pub fn start(workers: NonZeroU16) -> Result<(), AnalysisThreadsError> {
        let Self {
            started,
            workers: configured,
        } = PROCESS_POOL.get_or_init(|| Self {
            started: ThreadPoolBuilder::new()
                .num_threads(usize::from(workers.get()))
                .stack_size(DEFAULT_STACK_SIZE)
                .build_global()
                .map_err(Arc::new),
            workers,
        });

        started
            .clone()
            .map_err(|source| AnalysisThreadsError::WorkerPoolBuild {
                source,
                workers: *configured,
            })
            .and_then(|()| {
                (*configured == workers).then_some(()).ok_or(
                    AnalysisThreadsError::WorkerCountChanged {
                        configured: *configured,
                        requested: workers,
                    },
                )
            })
    }
}

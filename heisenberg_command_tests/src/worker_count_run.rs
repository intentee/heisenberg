use std::io;
use std::process::Child;

use crate::worker_count_outputs::WorkerCountOutputs;

pub struct WorkerCountRun {
    pub default_workers: Child,
    pub single_worker: Child,
}

impl WorkerCountRun {
    /// # Errors
    ///
    /// Returns the [`io::Error`] that keeps either run from finishing.
    pub fn outputs(self) -> io::Result<WorkerCountOutputs> {
        let Self {
            default_workers,
            single_worker,
        } = self;

        single_worker.wait_with_output().and_then(|single_worker| {
            default_workers
                .wait_with_output()
                .map(|default_workers| WorkerCountOutputs {
                    default_workers,
                    single_worker,
                })
        })
    }
}

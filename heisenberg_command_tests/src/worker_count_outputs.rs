use std::process::Output;

pub struct WorkerCountOutputs {
    pub default_workers: Output,
    pub single_worker: Output,
}

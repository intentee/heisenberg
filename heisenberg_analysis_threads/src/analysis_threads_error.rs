use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalysisThreadsError {
    #[error("the operating system cannot start a thread")]
    Spawn {
        #[from]
        source: io::Error,
    },
}

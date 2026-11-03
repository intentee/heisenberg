use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TestHarnessError {
    #[error("the heisenberg binary cannot be launched")]
    BinaryLaunch(#[source] io::Error),
}

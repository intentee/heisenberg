use std::env::VarError;
use std::io;

use clap::Error;
use thiserror::Error;

use crate::constants::CARGO_HOME_VARIABLE;
use crate::constants::SYSROOT_VARIABLE;

#[derive(Debug, Error)]
pub enum TestHarnessError {
    #[error("the heisenberg arguments are rejected")]
    Arguments(#[source] Error),
    #[error("the heisenberg binary cannot be launched")]
    BinaryLaunch(#[source] io::Error),
    #[error("{CARGO_HOME_VARIABLE} cannot be read")]
    CargoHome(#[source] VarError),
    #[error("{SYSROOT_VARIABLE} cannot be read")]
    Sysroot(#[source] VarError),
}

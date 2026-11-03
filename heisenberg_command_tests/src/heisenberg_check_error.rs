use std::env::VarError;

use clap::Error;
use thiserror::Error;

use crate::constants::CARGO_HOME_VARIABLE;
use crate::constants::SYSROOT_VARIABLE;

#[derive(Debug, Error)]
pub enum HeisenbergCheckError {
    #[error("the heisenberg arguments are rejected")]
    Arguments(#[source] Error),
    #[error("{CARGO_HOME_VARIABLE} cannot be read")]
    CargoHome(#[source] VarError),
    #[error("{SYSROOT_VARIABLE} cannot be read")]
    Sysroot(#[source] VarError),
}

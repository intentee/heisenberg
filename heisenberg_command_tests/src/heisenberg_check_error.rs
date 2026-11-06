use std::env::VarError;
use std::io;
use std::path::PathBuf;

use clap::Error;
use thiserror::Error;
use toml::de;
use toml::ser;

use heisenberg_error_context::contextual_error::ContextualError;

use crate::constants::CARGO_HOME_VARIABLE;
use crate::constants::SYSROOT_VARIABLE;

#[derive(Debug, Error)]
pub enum HeisenbergCheckError {
    #[error("the heisenberg arguments are rejected")]
    Arguments(#[source] Error),
    #[error("{CARGO_HOME_VARIABLE} cannot be read")]
    CargoHome(#[source] VarError),
    #[error("`{}` cannot be copied into the host-only fixture", .0.context.display())]
    FixtureCopy(#[source] ContextualError<PathBuf, io::Error>),
    #[error("the fixture manifest `{}` cannot be parsed", .0.context.display())]
    FixtureManifest(#[source] ContextualError<PathBuf, de::Error>),
    #[error("the host-only fixture manifest cannot be serialized")]
    FixtureManifestSerialization(#[source] ser::Error),
    #[error("the host-only fixture manifest `{}` cannot be written", .0.context.display())]
    FixtureManifestWrite(#[source] ContextualError<PathBuf, io::Error>),
    #[error("the previous host-only fixture `{}` cannot be removed", .0.context.display())]
    FixtureReset(#[source] ContextualError<PathBuf, io::Error>),
    #[error("{SYSROOT_VARIABLE} cannot be read")]
    Sysroot(#[source] VarError),
}

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
use crate::fixture_configuration::FixtureConfiguration;
use crate::fixture_out_dir::FixtureOutDir;

#[derive(Debug, Error)]
pub enum HeisenbergCheckError {
    #[error("the heisenberg arguments are rejected")]
    Arguments(#[source] Error),
    #[error("the build directory `{}` of the host-only fixture is not UTF-8", .0.display())]
    FixtureBuildDirectoryNotUtf8(PathBuf),
    #[error("the cargo configuration of the host-only fixture cannot be serialized")]
    FixtureCargoConfigurationSerialization(#[source] ser::Error),
    #[error("the cargo configuration `{}` of the host-only fixture cannot be written", .0.context.display())]
    FixtureCargoConfigurationWrite(#[source] ContextualError<PathBuf, io::Error>),
    #[error("{CARGO_HOME_VARIABLE} cannot be read")]
    CargoHome(#[source] VarError),
    #[error("the name of configuration {} of the fixture manifest `{}` is not a string", .0.context.index, .0.context.manifest_path.display())]
    FixtureConfigurationName(#[source] ContextualError<FixtureConfiguration, Box<de::Error>>),
    #[error("configuration {} of the fixture manifest `{}` has no name", .0.index, .0.manifest_path.display())]
    FixtureConfigurationNameMissing(FixtureConfiguration),
    #[error("the target of configuration {} of the fixture manifest `{}` is not a string", .0.context.index, .0.context.manifest_path.display())]
    FixtureConfigurationTarget(#[source] ContextualError<FixtureConfiguration, Box<de::Error>>),
    #[error("configuration {} of the fixture manifest `{}` has no target", .0.index, .0.manifest_path.display())]
    FixtureConfigurationTargetMissing(FixtureConfiguration),
    #[error("the configurations of the fixture manifest `{}` are not an array of tables", .0.context.display())]
    FixtureConfigurations(#[source] ContextualError<PathBuf, de::Error>),
    #[error("`{}` cannot be copied into the host-only fixture", .0.context.display())]
    FixtureCopy(#[source] ContextualError<PathBuf, io::Error>),
    #[error("the host-only fixture directory `{}` cannot be created", .0.context.display())]
    FixtureDirectoryCreate(#[source] ContextualError<PathBuf, io::Error>),
    #[error("the fixture manifest `{}` cannot be parsed", .0.context.display())]
    FixtureManifest(#[source] ContextualError<PathBuf, de::Error>),
    #[error("the fixture manifest `{}` cannot be read", .0.context.display())]
    FixtureManifestRead(#[source] ContextualError<PathBuf, io::Error>),
    #[error("the host-only fixture manifest cannot be serialized")]
    FixtureManifestSerialization(#[source] ser::Error),
    #[error("the host-only fixture manifest `{}` cannot be written", .0.context.display())]
    FixtureManifestWrite(#[source] ContextualError<PathBuf, io::Error>),
    #[error("the out-dir of `{}` in the fixture manifest `{}` declares configurations that are not a table", .0.context.package, .0.context.manifest_path.display())]
    FixtureOutDirConfigurations(#[source] ContextualError<FixtureOutDir, Box<de::Error>>),
    #[error("the out_dirs of the fixture manifest `{}` are not a table", .0.context.display())]
    FixtureOutDirs(#[source] ContextualError<PathBuf, de::Error>),
    #[error("the previous host-only fixture `{}` cannot be removed", .0.context.display())]
    FixtureReset(#[source] ContextualError<PathBuf, io::Error>),
    #[error("the fixture manifest `{}` has no [workspace.metadata.heisenberg] table", .0.display())]
    FixtureSettingsMissing(PathBuf),
    #[error("the modification time of `{}` cannot be given to its host-only fixture copy", .0.context.display())]
    FixtureTimestamp(#[source] ContextualError<PathBuf, io::Error>),
    #[error("{SYSROOT_VARIABLE} cannot be read")]
    Sysroot(#[source] VarError),
}

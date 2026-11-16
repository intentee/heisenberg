use std::env::VarError;
use std::error::Error as StdError;
use std::io;
use std::path::PathBuf;

use clap::Error;
use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8PathBuf;
use serde_json::error;
use thiserror::Error;
use toml::de;
use toml::ser;

use crate::constants::CARGO_HOME_VARIABLE;
use crate::constants::SYSROOT_VARIABLE;
use crate::fixture_configuration::FixtureConfiguration;
use crate::fixture_out_dir::FixtureOutDir;

#[derive(Debug, Error)]
pub enum HeisenbergCheckError {
    #[error("the heisenberg arguments are rejected")]
    Arguments(#[source] Error),
    #[error("the errors of the run cannot be reported")]
    ErrorReport(#[source] io::Error),
    #[error("the build directory `{}` of the host-only fixture is not UTF-8", .0.display())]
    FixtureBuildDirectoryNotUtf8(PathBuf),
    #[error("the cargo configuration of the host-only fixture cannot be serialized")]
    FixtureCargoConfigurationSerialization(#[source] ser::Error),
    #[error("the cargo configuration `{}` of the host-only fixture cannot be written", .0.display())]
    FixtureCargoConfigurationWrite(PathBuf, #[source] io::Error),
    #[error("{CARGO_HOME_VARIABLE} cannot be read")]
    CargoHome(#[source] VarError),
    #[error("the coverage export `{}` cannot be parsed", .0.display())]
    CoverageExportParse(PathBuf, #[source] error::Error),
    #[error("the coverage export `{}` cannot be read", .0.display())]
    CoverageExportRead(PathBuf, #[source] io::Error),
    #[error("the tampered coverage export `{}` cannot be serialized", .0.display())]
    CoverageExportSerialization(PathBuf, #[source] error::Error),
    #[error("the coverage export holds no value at `{pointer}` to tamper with")]
    CoverageExportValueMissing { pointer: &'static str },
    #[error("the tampered coverage export `{}` cannot be written", .0.display())]
    CoverageExportWrite(PathBuf, #[source] io::Error),
    #[error("the name of configuration {} of the fixture manifest `{}` is not a string", .0.index, .0.manifest_path.display())]
    FixtureConfigurationName(FixtureConfiguration, #[source] Box<de::Error>),
    #[error("configuration {} of the fixture manifest `{}` has no name", .0.index, .0.manifest_path.display())]
    FixtureConfigurationNameMissing(FixtureConfiguration),
    #[error("the target of configuration {} of the fixture manifest `{}` is not a string", .0.index, .0.manifest_path.display())]
    FixtureConfigurationTarget(FixtureConfiguration, #[source] Box<de::Error>),
    #[error("configuration {} of the fixture manifest `{}` has no target", .0.index, .0.manifest_path.display())]
    FixtureConfigurationTargetMissing(FixtureConfiguration),
    #[error("the configurations of the fixture manifest `{}` are not an array of tables", .0.display())]
    FixtureConfigurations(PathBuf, #[source] de::Error),
    #[error("`{}` cannot be copied into the host-only fixture", .0.display())]
    FixtureCopy(PathBuf, #[source] io::Error),
    #[error("`{}` cannot be created to be cleared", .0.display())]
    ClearedPathCreate(PathBuf, #[source] io::Error),
    #[error("the fixture manifest `{}` cannot be parsed", .0.display())]
    FixtureManifest(PathBuf, #[source] de::Error),
    #[error("the fixture manifest `{}` cannot be read", .0.display())]
    FixtureManifestRead(PathBuf, #[source] io::Error),
    #[error("the host-only fixture manifest cannot be serialized")]
    FixtureManifestSerialization(#[source] ser::Error),
    #[error("the host-only fixture manifest `{}` cannot be written", .0.display())]
    FixtureManifestWrite(PathBuf, #[source] io::Error),
    #[error("the out-dir of `{}` in the fixture manifest `{}` declares configurations that are not a table", .0.package, .0.manifest_path.display())]
    FixtureOutDirConfigurations(FixtureOutDir, #[source] Box<de::Error>),
    #[error("the out_dirs of the fixture manifest `{}` are not a table", .0.display())]
    FixtureOutDirs(PathBuf, #[source] de::Error),
    #[error("`{}` cannot be cleared", .0.display())]
    ClearedPathRemoval(PathBuf, #[source] io::Error),
    #[error("the fixture manifest `{}` has no [workspace.metadata.heisenberg] table", .0.display())]
    FixtureSettingsMissing(PathBuf),
    #[error("the modification time of `{}` cannot be given to its host-only fixture copy", .0.display())]
    FixtureTimestamp(PathBuf, #[source] io::Error),
    #[error("the procedural macro server of the sysroot cannot be found")]
    ProcMacroServerMissing(#[source] Box<dyn StdError + Send + Sync>),
    #[error("the procedural macro server of the sysroot cannot be started")]
    ProcMacroServerSpawn(#[source] io::Error),
    #[error("{SYSROOT_VARIABLE} cannot be read")]
    Sysroot(#[source] VarError),
    #[error("{SYSROOT_VARIABLE} holds `{0}`, which is not an absolute path")]
    SysrootNotAbsolute(Utf8PathBuf),
    #[error("the sysroot `{0}` has no root directory")]
    SysrootWithoutRoot(AbsPathBuf),
}

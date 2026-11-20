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
    #[error("the cargo configuration `{}` of the host-only fixture cannot be written", .path.display())]
    FixtureCargoConfigurationWrite {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("{CARGO_HOME_VARIABLE} cannot be read")]
    CargoHome(#[source] VarError),
    #[error("the coverage export `{}` cannot be parsed", .path.display())]
    CoverageExportParse {
        path: PathBuf,
        #[source]
        source: error::Error,
    },
    #[error("the coverage export `{}` cannot be read", .path.display())]
    CoverageExportRead {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("the tampered coverage export `{}` cannot be serialized", .path.display())]
    CoverageExportSerialization {
        path: PathBuf,
        #[source]
        source: error::Error,
    },
    #[error("the coverage export lists no function of `{}` to tamper with", .file.display())]
    CoverageExportFunctionMissing { file: PathBuf },
    #[error("the coverage export holds no value at `{pointer}` to tamper with")]
    CoverageExportValueMissing { pointer: &'static str },
    #[error("the tampered coverage export `{}` cannot be written", .path.display())]
    CoverageExportWrite {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("the name of configuration {} of the fixture manifest `{}` is not a string", .configuration.index, .configuration.manifest_path.display())]
    FixtureConfigurationName {
        configuration: FixtureConfiguration,
        #[source]
        source: Box<de::Error>,
    },
    #[error("configuration {} of the fixture manifest `{}` has no name", .0.index, .0.manifest_path.display())]
    FixtureConfigurationNameMissing(FixtureConfiguration),
    #[error("the target of configuration {} of the fixture manifest `{}` is not a string", .configuration.index, .configuration.manifest_path.display())]
    FixtureConfigurationTarget {
        configuration: FixtureConfiguration,
        #[source]
        source: Box<de::Error>,
    },
    #[error("configuration {} of the fixture manifest `{}` has no target", .0.index, .0.manifest_path.display())]
    FixtureConfigurationTargetMissing(FixtureConfiguration),
    #[error("the configurations of the fixture manifest `{}` are not an array of tables", .path.display())]
    FixtureConfigurations {
        path: PathBuf,
        #[source]
        source: de::Error,
    },
    #[error("`{}` cannot be copied into the host-only fixture", .path.display())]
    FixtureCopy {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("`{}` cannot be created to be cleared", .path.display())]
    ClearedPathCreate {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("the fixture manifest `{}` cannot be parsed", .path.display())]
    FixtureManifest {
        path: PathBuf,
        #[source]
        source: de::Error,
    },
    #[error("the fixture manifest `{}` cannot be read", .path.display())]
    FixtureManifestRead {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("the host-only fixture manifest cannot be serialized")]
    FixtureManifestSerialization(#[source] ser::Error),
    #[error("the host-only fixture manifest `{}` cannot be written", .path.display())]
    FixtureManifestWrite {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("the out-dir of `{}` in the fixture manifest `{}` declares configurations that are not a table", .out_dir.package, .out_dir.manifest_path.display())]
    FixtureOutDirConfigurations {
        out_dir: FixtureOutDir,
        #[source]
        source: Box<de::Error>,
    },
    #[error("the out-dir `{}` of a configuration that the host-only fixture leaves out cannot be removed", .path.display())]
    FixtureOutDirRemoval {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("the out_dirs of the fixture manifest `{}` are not a table", .path.display())]
    FixtureOutDirs {
        path: PathBuf,
        #[source]
        source: de::Error,
    },
    #[error("`{}` cannot be cleared", .path.display())]
    ClearedPathRemoval {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("the fixture manifest `{}` has no [workspace.metadata.heisenberg] table", .0.display())]
    FixtureSettingsMissing(PathBuf),
    #[error("the modification time of `{}` cannot be given to its host-only fixture copy", .path.display())]
    FixtureTimestamp {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
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

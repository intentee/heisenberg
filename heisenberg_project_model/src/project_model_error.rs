use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;

use cargo::context::Definition;
use cargo_platform::ParseError;
use guppy::errors;
use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8PathBuf;
use ra_ap_vfs::VfsPath;
use serde_json::error;
use thiserror::Error;
use toml::de;

use heisenberg_error_context::contextual_error::ContextualError;
use heisenberg_module_graph::crate_target::CrateTarget;

use crate::constants::CARGO_ENVIRONMENT_TABLE;
use crate::constants::CLIPPY_ARGUMENTS_VARIABLE;
use crate::constants::CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE;
use crate::constants::NIGHTLY_BOOTSTRAP_VARIABLE;
use crate::constants::SUPPORTED_TARGETS;
use crate::supported_target::SupportedTarget;

#[derive(Debug, Error)]
pub enum ProjectModelError {
    #[error("`{manifest_path}` declares the target `{target}` more than once")]
    DuplicateTarget {
        manifest_path: AbsPathBuf,
        target: String,
    },
    #[error("the [workspace.metadata.heisenberg] settings of `{manifest_path}` are invalid")]
    HeisenbergSettingsInvalid {
        manifest_path: AbsPathBuf,
        #[source]
        source: Box<de::Error>,
    },
    #[error("`{manifest_path}` declares no [workspace.metadata.heisenberg] settings")]
    HeisenbergSettingsMissing { manifest_path: AbsPathBuf },
    #[error("`{manifest_path}` declares no targets in [workspace.metadata.heisenberg]")]
    NoTargets { manifest_path: AbsPathBuf },
    #[error(
        "`{target}` in `{manifest_path}` is not a supported target; the supported targets are {}",
        SUPPORTED_TARGETS.map(SupportedTarget::triple).join(", ")
    )]
    UnknownTarget {
        manifest_path: AbsPathBuf,
        target: String,
    },
    #[error("the cargo configuration of `{manifest_path}` cannot be loaded")]
    CargoConfiguration {
        manifest_path: PathBuf,
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },
    #[error("the cargo workspace `{manifest_path}` cannot be loaded")]
    CargoWorkspace {
        manifest_path: PathBuf,
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },
    #[error("the dependencies of `{manifest_path}` cannot be resolved from its lockfile")]
    DependencyResolution {
        manifest_path: PathBuf,
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },
    #[error("`{package}` depends on `{dependency}` under more than one crate name")]
    ExternCrateName {
        dependency: String,
        package: String,
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },
    #[error("the clippy configuration `{path}` is not valid TOML")]
    ClippyConfigurationInvalid {
        path: PathBuf,
        #[source]
        source: Box<de::Error>,
    },
    #[error("the clippy configuration `{path}` cannot be read")]
    ClippyConfigurationUnreadable {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("the `{key}` value of the cargo configuration of `{manifest_path}` cannot be read")]
    CargoConfigurationValue {
        key: String,
        manifest_path: PathBuf,
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },
    #[error(
        "`{variable}` is set in the `[{table}]` table of {definition}, and Heisenberg cannot account for a clippy configuration outside the workspace",
        table = CARGO_ENVIRONMENT_TABLE,
        variable = CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE
    )]
    ConfiguredClippyConfigurationDirectory { definition: Definition },
    #[error(
        "`{variable}` is set in the `[{table}]` table of {definition}, and Heisenberg cannot account for nightly features",
        table = CARGO_ENVIRONMENT_TABLE,
        variable = NIGHTLY_BOOTSTRAP_VARIABLE
    )]
    ConfiguredNightlyBootstrap { definition: Definition },
    #[error(
        "`{variable}` is set in the `[{table}]` table of {definition}, and Heisenberg cannot account for clippy arguments",
        table = CARGO_ENVIRONMENT_TABLE,
        variable = CLIPPY_ARGUMENTS_VARIABLE
    )]
    ConfiguredClippyArguments { definition: Definition },
    #[error("`{key}` is set in {definition}, and Heisenberg cannot account for rustflags")]
    ConfiguredRustflags { definition: Definition, key: String },
    #[error(
        "`{variable}` is set in the environment, and Heisenberg cannot account for a clippy configuration outside the workspace",
        variable = CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE
    )]
    EnvironmentClippyConfigurationDirectory,
    #[error(
        "`{variable}` is set in the environment, and Heisenberg cannot account for clippy arguments",
        variable = CLIPPY_ARGUMENTS_VARIABLE
    )]
    EnvironmentClippyArguments,
    #[error(
        "`{variable}` is set in the environment, and Heisenberg cannot account for nightly features"
    )]
    EnvironmentNightlyFeatures { variable: &'static str },
    #[error("`{variable}` is set in the environment, and Heisenberg cannot account for rustflags")]
    EnvironmentRustflags { variable: &'static str },
    #[error("the embedded crate cfg table of the target is not `rustc --print cfg` output")]
    CratesCfgTable(#[source] ParseError),
    #[error("crate root `{crate_root}` lies outside the workspace root `{workspace_root}`")]
    CrateRootOutsideWorkspace {
        crate_root: PathBuf,
        workspace_root: PathBuf,
    },
    #[error("the crate root `{}` cannot be read", .0.context.display())]
    CrateRootUnreadable(#[source] ContextualError<PathBuf, io::Error>),
    #[error(
        "the embedded standard library cfg table of the target is not `rustc --print cfg` output"
    )]
    LibraryCfgTable(#[source] ParseError),
    #[error("guppy cannot resolve the standard library features for the target")]
    LibraryFeatureResolution(#[source] Box<errors::Error>),
    #[error("cargo_metadata rejects the standard library metadata document built for the target")]
    LibraryMetadata(#[source] error::Error),
    #[error("guppy rejects the standard library metadata document")]
    LibraryPackageGraph(#[source] Box<errors::Error>),
    #[error("rust-analyzer rejects the standard library rust-project.json document")]
    LibraryProjectJson(#[source] error::Error),
    #[error("rust-analyzer returns no standard library workspace for `{library_directory}`")]
    LibraryWorkspaceMissing { library_directory: AbsPathBuf },
    #[error("the [env] table of the cargo configuration cannot be read")]
    CargoEnvironmentTable(#[source] Box<dyn Error + Send + Sync>),
    #[error("the configured value of `{}` is not UTF-8", .0.context)]
    NonUtf8ConfiguredEnvironmentValue(ContextualError<String, OsString>),
    #[error(
        "`{manifest_path}` declares an out-dir for `{package}`, which is not a workspace member"
    )]
    UnknownOutDirPackage {
        manifest_path: AbsPathBuf,
        package: String,
    },
    #[error("`{manifest_path}` declares an out-dir for `{package}`, which has no build script")]
    OutDirPackageWithoutBuildScript {
        manifest_path: AbsPathBuf,
        package: String,
    },
    #[error(
        "`{manifest_path}` declares the out-dir `{directory}` for `{package}` outside the workspace"
    )]
    OutDirOutsideWorkspace {
        directory: String,
        manifest_path: AbsPathBuf,
        package: String,
    },
    #[error("the declared out-dir `{}` cannot be read", .0.context.display())]
    UnreadableOutDir(#[source] ContextualError<PathBuf, io::Error>),
    #[error("the dependency sources of `{manifest_path}` cannot be loaded")]
    PackageSources {
        manifest_path: PathBuf,
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },
    #[error("{target} of package `{package}` is a metabuild script, which has no source file")]
    MetabuildTarget { package: String, target: String },
    #[error(
        "workspace member `{member_manifest_path}` lies outside the workspace root `{workspace_root}`"
    )]
    MemberOutsideWorkspace {
        member_manifest_path: PathBuf,
        workspace_root: PathBuf,
    },
    #[error("`{workspace_directory}` is not the root of its cargo workspace `{workspace_root}`")]
    NotTheWorkspaceRoot {
        workspace_directory: PathBuf,
        workspace_root: PathBuf,
    },
    #[error("guppy cannot resolve the project features for the target")]
    ProjectFeatureResolution(#[source] Box<errors::Error>),
    #[error("cargo_metadata rejects the project metadata document built for the target")]
    ProjectMetadata(#[source] error::Error),
    #[error("the environment variable name {0:?} is not UTF-8")]
    NonUtf8EnvironmentVariableName(OsString),
    #[error("the value {value:?} of the environment variable `{name}` is not UTF-8")]
    NonUtf8EnvironmentVariableValue { name: String, value: OsString },
    #[error("the workspace root `{}` is not a UTF-8 path", .0.display())]
    NonUtf8WorkspaceRoot(PathBuf),
    #[error("the target directory `{}` is not a UTF-8 path", .0.display())]
    NonUtf8TargetDirectory(PathBuf),
    #[error("the crate root `{}` is not a UTF-8 path", .0.display())]
    NonUtf8CrateRoot(PathBuf),
    #[error("the workspace root `{0}` is not an absolute path")]
    RelativeWorkspaceRoot(Utf8PathBuf),
    #[error("the target directory `{0}` is not an absolute path")]
    RelativeTargetDirectory(Utf8PathBuf),
    #[error("the crate root `{0}` is not an absolute path")]
    RelativeCrateRoot(Utf8PathBuf),
    #[error("cargo_metadata cannot read the metadata cargo produces for package `{}`", .0.context)]
    PackageMetadata(#[source] ContextualError<String, error::Error>),
    #[error("guppy rejects the project metadata document")]
    ProjectPackageGraph(#[source] Box<errors::Error>),
    #[error("crate root `{crate_root}` belongs to both {first} and {second}")]
    SharedCrateRoot {
        crate_root: PathBuf,
        first: Box<CrateTarget>,
        second: Box<CrateTarget>,
    },
    #[error("target-spec rejects the embedded crate cfg table of the target")]
    TargetPlatform(#[source] errors::TargetSpecError),
    #[error("rust-analyzer holds `{0}`, which is not a file on disk")]
    VirtualSourceFile(VfsPath),
    #[error("the workspace manifest `{0}` has no parent directory")]
    WorkspaceManifestPath(AbsPathBuf),
    #[error("rust-analyzer cannot load the workspace for the target")]
    WorkspaceLoad(#[source] Box<dyn Error + Send + Sync>),
}

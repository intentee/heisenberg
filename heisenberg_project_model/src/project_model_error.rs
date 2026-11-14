use std::collections::BTreeSet;
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;
use std::str::Utf8Error;
use std::sync::Arc;

use cargo::context::Definition;
use cargo_platform::ParseError;
use guppy::errors;
use ra_ap_ide_db::base_db::ProcMacroLoadingError;
use ra_ap_paths::AbsPathBuf;
use ra_ap_paths::Utf8PathBuf;
use ra_ap_vfs::VfsPath;
use serde_json::error;
use thiserror::Error;
use toml::de;
use walkdir::Error as WalkError;

use heisenberg_analysis_threads::analysis_threads_error::AnalysisThreadsError;
use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_workspace_build::workspace_build_error::WorkspaceBuildError;

use crate::configuration::Configuration;
use crate::configuration_list::ConfigurationList;
use crate::constants::CARGO_ENVIRONMENT_TABLE;
use crate::constants::CLIPPY_ARGUMENTS_VARIABLE;
use crate::constants::CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE;
use crate::constants::NEXTEST_RETRIES_VARIABLE;
use crate::constants::NIGHTLY_BOOTSTRAP_VARIABLE;
use crate::supported_target_list::SupportedTargetList;

#[derive(Debug, Error)]
pub enum ProjectModelError {
    #[error(
        "`{manifest_path}` declares the configurations `{first}` and `{second}` with the same target and features"
    )]
    ConfigurationsAnalyseTheSameBuild {
        first: String,
        manifest_path: AbsPathBuf,
        second: String,
    },
    #[error("`{manifest_path}` declares the configuration `{configuration}` more than once")]
    ConfigurationDeclaredTwice {
        configuration: String,
        manifest_path: AbsPathBuf,
    },
    #[error(
        "the configuration `{configuration}` of `{manifest_path}` lists the feature `{feature}` more than once"
    )]
    ConfigurationFeatureListedTwice {
        configuration: String,
        feature: String,
        manifest_path: AbsPathBuf,
    },
    #[error(
        "`{manifest_path}` uses resolver 1 with a root package and other members, so cargo would apply the feature selection of the configuration `{configuration}` to the root package only"
    )]
    ConfigurationFeaturesReachOnlyTheRootPackage {
        configuration: String,
        manifest_path: AbsPathBuf,
    },
    #[error("`{configuration}` is selected twice with --configuration")]
    ConfigurationSelectedTwice { configuration: String },
    #[error("`{manifest_path}` declares the configuration at index {index} without a name")]
    ConfigurationWithoutName {
        index: usize,
        manifest_path: AbsPathBuf,
    },
    #[error("the [workspace.metadata.heisenberg] settings of `{manifest_path}` are invalid")]
    HeisenbergSettingsInvalid {
        manifest_path: AbsPathBuf,
        #[source]
        source: Box<de::Error>,
    },
    #[error("`{manifest_path}` declares no [workspace.metadata.heisenberg] settings")]
    HeisenbergSettingsMissing { manifest_path: AbsPathBuf },
    #[error("`{manifest_path}` declares no configurations in [workspace.metadata.heisenberg]")]
    NoConfigurations { manifest_path: AbsPathBuf },
    #[error(
        "`{configuration}` is selected with --configuration, but `{manifest_path}` does not declare it; the declared configurations are {}",
        ConfigurationList { configurations: declared }
    )]
    SelectedConfigurationNotDeclared {
        configuration: String,
        declared: BTreeSet<Configuration>,
        manifest_path: AbsPathBuf,
    },
    #[error(
        "the configuration `{configuration}` of `{manifest_path}` selects the feature `{feature}`, which no workspace member defines"
    )]
    UnknownConfigurationFeature {
        configuration: String,
        feature: String,
        manifest_path: AbsPathBuf,
    },
    #[error(
        "the configuration `{configuration}` of `{manifest_path}` targets `{target}`, which is not a supported target; the supported targets are {}",
        SupportedTargetList
    )]
    UnknownConfigurationTarget {
        configuration: String,
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
    #[error("the nextest configuration `{path}` is not valid")]
    NextestConfigurationInvalid {
        path: PathBuf,
        #[source]
        source: Box<de::Error>,
    },
    #[error("the nextest configuration `{path}` cannot be read")]
    NextestConfigurationUnreadable {
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
    #[error("`{key}` is set in {definition}, and Heisenberg builds with the rustc of the sysroot")]
    ConfiguredCompiler { definition: Definition, key: String },
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
        "`{variable}` is set in the environment, and Heisenberg cannot account for tests that nextest retries",
        variable = NEXTEST_RETRIES_VARIABLE
    )]
    EnvironmentNextestRetries,
    #[error(
        "`{variable}` is set in the environment, and Heisenberg cannot account for nightly features"
    )]
    EnvironmentNightlyFeatures { variable: &'static str },
    #[error(
        "`{variable}` is set in the environment, and Heisenberg builds with the rustc of the sysroot"
    )]
    EnvironmentCompiler { variable: &'static str },
    #[error("`{variable}` is set in the environment, and Heisenberg cannot account for rustflags")]
    EnvironmentRustflags { variable: &'static str },
    #[error("the embedded crate cfg table of the target is not `rustc --print cfg` output")]
    CratesCfgTable(#[source] ParseError),
    #[error("crate root `{crate_root}` lies outside the workspace root `{workspace_root}`")]
    CrateRootOutsideWorkspace {
        crate_root: PathBuf,
        workspace_root: PathBuf,
    },
    #[error("the crate root `{}` cannot be read", .path.display())]
    CrateRootUnreadable {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("`{}` is not UTF-8, so rust-analyzer loads it without text", .path)]
    SourceFileNotUtf8 {
        path: AbsPathBuf,
        #[source]
        source: Utf8Error,
    },
    #[error("`{}` cannot be read to confirm that rust-analyzer loaded its text", .path)]
    SourceFileUnreadable {
        path: AbsPathBuf,
        #[source]
        source: io::Error,
    },
    #[error(transparent)]
    SourceFileUnavailable(Arc<Self>),
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
    #[error("the configured value of `{}` is not UTF-8", .variable)]
    NonUtf8ConfiguredEnvironmentValue { variable: String, value: OsString },
    #[error(
        "`{manifest_path}` declares an out-dir for `{package}`, which is not a package of the workspace"
    )]
    UnknownOutDirPackage {
        manifest_path: AbsPathBuf,
        package: String,
    },
    #[error(
        "`{manifest_path}` declares an out-dir for `{package}`, which names {} packages of the workspace: {}",
        versions.len(),
        versions.join(", ")
    )]
    AmbiguousOutDirPackage {
        manifest_path: AbsPathBuf,
        package: String,
        versions: Vec<String>,
    },
    #[error(
        "`{manifest_path}` declares an out-dir for `{package}` without the build script output of the configuration `{configuration}`"
    )]
    OutDirConfigurationUndeclared {
        configuration: String,
        manifest_path: AbsPathBuf,
        package: String,
    },
    #[error(
        "`{manifest_path}` declares an out-dir for `{package}` with the build script output of `{configuration}`, which is not a declared configuration"
    )]
    UnknownOutDirConfiguration {
        configuration: String,
        manifest_path: AbsPathBuf,
        package: String,
    },
    #[error("`{manifest_path}` declares an out-dir for `{package}`, which has no build script")]
    OutDirPackageWithoutBuildScript {
        manifest_path: AbsPathBuf,
        package: String,
    },
    #[error(
        "`{manifest_path}` declares the out-dir `{directory}` of `{package}` for the configuration `{configuration}` outside the workspace"
    )]
    OutDirOutsideWorkspace {
        configuration: String,
        directory: String,
        manifest_path: AbsPathBuf,
        package: String,
    },
    #[error("the build scripts and procedural macros of the workspace cannot be built for the configuration `{}`", .configuration.name)]
    WorkspaceBuild {
        configuration: Configuration,
        #[source]
        source: Box<WorkspaceBuildError>,
    },
    #[error("cargo cannot prepare the build of the build scripts and procedural macros")]
    WorkspaceBuildSession(#[source] WorkspaceBuildError),
    #[error("the embedded target specification cannot be read")]
    TargetSpec(#[source] error::Error),
    #[error("the build script output cfg `{}` cannot be parsed", .cfg)]
    BuildScriptCfg {
        cfg: String,
        #[source]
        source: ParseError,
    },
    #[error("the build script output `{out_dir}` is not a directory")]
    BuildScriptOutputNotADirectory { out_dir: AbsPathBuf },
    #[error("the build script output in `{out_dir}` cannot be listed")]
    UnlistableBuildScriptOutput {
        out_dir: AbsPathBuf,
        #[source]
        source: io::Error,
    },
    #[error("the build script output cannot be walked")]
    UnwalkableBuildScriptOutput(#[source] WalkError),
    #[error("the build script output `{}` cannot be read", .path.display())]
    UnreadableBuildScriptOutput {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error(
        "the build script of `{package}` writes `{}` into its OUT_DIR, and that name is not UTF-8",
        name.display()
    )]
    NonUtf8BuildScriptOutputName {
        name: PathBuf,
        out_dir: AbsPathBuf,
        package: String,
    },
    #[error("the build script output `{0}` is not an absolute path")]
    RelativeBuildScriptOutput(Utf8PathBuf),
    #[error("the procedural macro crate rooted at `{0}` has no built library")]
    ProcMacroLibraryMissing(VfsPath),
    #[error("the procedural macros of `{}` cannot be loaded", .path)]
    ProcMacroLoad {
        path: AbsPathBuf,
        #[source]
        source: ProcMacroLoadingError,
    },
    #[error("the procedural macros of `{}` cannot be loaded on a thread of their own", .path)]
    ProcMacroLoadThread {
        path: AbsPathBuf,
        #[source]
        source: AnalysisThreadsError,
    },
    #[error("the sysroot has no rustc in `{directory}`")]
    SysrootWithoutCompiler { directory: AbsPathBuf },
    #[error("the sysroot has no procedural macro server")]
    ProcMacroServerMissing(#[source] Box<dyn Error + Send + Sync>),
    #[error("the procedural macro server `{}` cannot be started", .path)]
    ProcMacroServerSpawn {
        path: AbsPathBuf,
        #[source]
        source: io::Error,
    },
    #[error(transparent)]
    ProcMacroServerUnavailable(Arc<Self>),
    #[error(transparent)]
    BuildScriptOutputUnavailable(Arc<Self>),
    #[error(transparent)]
    TargetModelUnavailable(Arc<Self>),
    #[error("the standard library resolution thread cannot be started")]
    LibraryResolutionThread(#[source] AnalysisThreadsError),
    #[error("the declared out-dir `{}` cannot be read", .path.display())]
    UnreadableOutDir {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
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
    #[error("cargo_metadata cannot read the metadata cargo produces for package `{}`", .package)]
    PackageMetadata {
        package: String,
        #[source]
        source: error::Error,
    },
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

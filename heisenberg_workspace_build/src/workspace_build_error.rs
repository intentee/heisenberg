use std::error::Error;
use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceBuildError {
    #[error("cargo cannot plan the build of the build scripts and procedural macros")]
    BuildPlan(#[source] Box<dyn Error + Send + Sync>),
    #[error("cargo cannot configure the build of the build scripts and procedural macros")]
    BuildConfiguration(#[source] Box<dyn Error + Send + Sync>),
    #[error("the build scripts and procedural macros cannot be built or run:\n{output}")]
    Build {
        output: String,
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },
    #[error("the messages of the build cannot be read")]
    BuildMessages(#[source] io::Error),
    #[error("the cargo configuration of the build cannot be read")]
    CargoConfiguration(#[source] Box<dyn Error + Send + Sync>),
    #[error("the cargo workspace of the build cannot be loaded")]
    CargoWorkspace(#[source] Box<dyn Error + Send + Sync>),
    #[error("cargo cannot name the files of the procedural macros")]
    ProcMacroFileTypes(#[source] Box<dyn Error + Send + Sync>),
    #[error("the standard library of {target} is not installed at `{}`", directory.display())]
    TargetLibraryMissing { directory: PathBuf, target: String },
    #[error(
        "the build script of `{dependent}` needs the build script of `{substituted}`, which out_dirs declares instead of running it"
    )]
    SubstitutedBuildScriptRequired {
        dependent: String,
        substituted: String,
    },
}

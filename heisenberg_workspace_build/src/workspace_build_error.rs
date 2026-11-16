use std::error::Error;
use std::io;
use std::path::PathBuf;

use cargo_metadata::camino::Utf8Path;
use cargo_metadata::camino::Utf8PathBuf;

use crate::unparsable_file::UnparsableFile;

fn rendered_build_output(output: &Result<String, io::Error>) -> String {
    match output {
        Ok(diagnostics) => diagnostics.clone(),
        Err(unreadable) => format!("the messages of the build cannot be read: {unreadable}"),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceBuildError {
    #[error("cargo cannot plan the build of the build scripts and procedural macros")]
    BuildPlan(#[source] Box<dyn Error + Send + Sync>),
    #[error("cargo cannot configure the build of the build scripts and procedural macros")]
    BuildConfiguration(#[source] Box<dyn Error + Send + Sync>),
    #[error(
        "the build scripts and procedural macros cannot be built or run; a package whose build script cannot run on every host can declare its output in [workspace.metadata.heisenberg.out_dirs.<package>] instead:\n{}",
        rendered_build_output(output)
    )]
    Build {
        output: Result<String, io::Error>,
        #[source]
        source: Box<dyn Error + Send + Sync>,
    },
    #[error("the messages of the build cannot be read")]
    BuildMessages(#[source] io::Error),
    #[error("the cargo configuration of the build cannot be read")]
    CargoConfiguration(#[source] Box<Self>),
    #[error(transparent)]
    CargoContext(Box<dyn Error + Send + Sync>),
    #[error("the cargo workspace of the build cannot be loaded")]
    CargoWorkspace(#[source] Box<dyn Error + Send + Sync>),
    #[error(
        "the files that cargo lists for the procedural macro `{proc_macro}` hold {} dynamic libraries instead of one: [{}]; the files that are not object files: [{}]",
        libraries.len(),
        libraries.iter().map(Utf8PathBuf::as_path).map(Utf8Path::as_str).collect::<Vec<&str>>().join(", "),
        unparsable.iter().map(ToString::to_string).collect::<Vec<String>>().join(", ")
    )]
    ProcMacroLibraryNotUnique {
        libraries: Vec<Utf8PathBuf>,
        proc_macro: String,
        unparsable: Vec<UnparsableFile>,
    },
    #[error(
        "the file `{file}` that cargo lists for the procedural macro `{proc_macro}` cannot be read"
    )]
    ProcMacroLibraryRead {
        file: Utf8PathBuf,
        proc_macro: String,
        #[source]
        source: io::Error,
    },
    #[error("the standard library of {target} is not installed at `{}`", directory.display())]
    TargetLibraryMissing { directory: PathBuf, target: String },
    #[error(
        "the build script of `{dependent}` needs the build script of `{substituted}`, which out_dirs declares instead of running it; declare an out-dir for `{dependent}` too"
    )]
    SubstitutedBuildScriptRequired {
        dependent: String,
        substituted: String,
    },
    #[error(
        "the build scripts and procedural macros of the workspace compile `{package}`, which needs its build script to run, and out_dirs declares its output instead of running it; a package that build scripts or procedural macros depend on cannot be declared in out_dirs"
    )]
    SubstitutedPackageBuilt { package: String },
}

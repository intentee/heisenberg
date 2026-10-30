use std::error::Error;
use std::io;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectModelError {
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
    #[error("the library root `{path}` cannot be read")]
    LibraryRootUnreadable {
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
    #[error(
        "workspace member `{member_manifest_path}` lies outside the workspace root `{workspace_root}`"
    )]
    MemberOutsideWorkspace {
        member_manifest_path: PathBuf,
        workspace_root: PathBuf,
    },
}

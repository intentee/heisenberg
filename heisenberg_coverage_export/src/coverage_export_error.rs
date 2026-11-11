use std::io;

use ra_ap_paths::AbsPathBuf;
use serde_json::error;
use thiserror::Error;

use heisenberg_absolute_path::absolute_path_error::AbsolutePathError;
use heisenberg_error_context::contextual_error::ContextualError;

use crate::constants::EXPORT_FORMAT_VERSION;
use crate::constants::EXPORT_TYPE;
use crate::region_position::RegionPosition;

#[derive(Debug, Error)]
pub enum CoverageExportError {
    #[error(
        "the function `{function}` of the coverage export `{export}` records branch coverage, which Heisenberg does not measure; export the coverage of a run without branch coverage"
    )]
    BranchCoverage {
        export: AbsPathBuf,
        function: String,
    },
    #[error(
        "the region from {}:{} to {}:{} of the function `{function}` in the coverage export `{export}` has a position before the first line or column; export the coverage with cargo llvm-cov",
        .start.line,
        .start.column,
        .end.line,
        .end.column
    )]
    InvalidRegionPosition {
        end: RegionPosition,
        export: AbsPathBuf,
        function: String,
        start: RegionPosition,
    },
    #[error("the coverage export `{}` is not valid JSON", .0.context)]
    MalformedJson(#[source] ContextualError<AbsPathBuf, error::Error>),
    #[error(
        "the function `{function}` of the coverage export `{export}` records MC/DC coverage, which Heisenberg does not measure; export the coverage of a run without MC/DC coverage"
    )]
    McdcCoverage {
        export: AbsPathBuf,
        function: String,
    },
    #[error("the metadata of the coverage export `{}` cannot be read", .0.context)]
    MetadataUnreadable(#[source] ContextualError<AbsPathBuf, io::Error>),
    #[error("the modification time of the coverage export `{}` is not available", .0.context)]
    ModificationTimeUnavailable(#[source] ContextualError<AbsPathBuf, io::Error>),
    #[error(
        "the coverage export `{}` does not name the type and version of an llvm-cov JSON export",
        .0.context
    )]
    NotAnExport(#[source] ContextualError<AbsPathBuf, error::Error>),
    #[error(
        "the coverage export `{export}` covers the workspace of `{manifest_path}`, not the workspace of `{workspace_manifest}`"
    )]
    OtherWorkspace {
        export: AbsPathBuf,
        manifest_path: AbsPathBuf,
        workspace_manifest: AbsPathBuf,
    },
    #[error(
        "the region from {}:{} to {}:{} of the function `{function}` in the coverage export `{export}` ends before it starts; export the coverage with cargo llvm-cov",
        .start.line,
        .start.column,
        .end.line,
        .end.column
    )]
    RegionEndBeforeStart {
        end: RegionPosition,
        export: AbsPathBuf,
        function: String,
        start: RegionPosition,
    },
    #[error(
        "a region of the function `{function}` in the coverage export `{export}` refers to file {file_id}, past the end of its list of filenames, which holds {listed}"
    )]
    RegionFileOutOfRange {
        export: AbsPathBuf,
        file_id: usize,
        function: String,
        listed: usize,
    },
    #[error(
        "the coverage export `{export}` names a relative workspace manifest; export the coverage with cargo llvm-cov"
    )]
    RelativeManifestPath {
        export: AbsPathBuf,
        #[source]
        source: AbsolutePathError,
    },
    #[error(
        "the function `{function}` of the coverage export `{export}` names a relative file; export the coverage of a run without --remap-path-prefix"
    )]
    RelativeRegionFile {
        export: AbsPathBuf,
        function: String,
        #[source]
        source: AbsolutePathError,
    },
    #[error(
        "the coverage export `{}` does not hold the functions and regions of a `cargo llvm-cov report --json` export made without --summary-only and --skip-functions",
        .0.context
    )]
    SchemaMismatch(#[source] ContextualError<AbsPathBuf, error::Error>),
    #[error("the coverage export `{}` cannot be opened", .0.context)]
    Unopenable(#[source] ContextualError<AbsPathBuf, io::Error>),
    #[error("the coverage export `{}` cannot be read", .0.context)]
    Unreadable(#[source] ContextualError<AbsPathBuf, io::Error>),
    #[error(
        "the coverage export `{export}` is of type `{export_type}` in version {version}; Heisenberg reads `{}` in version {}",
        EXPORT_TYPE,
        EXPORT_FORMAT_VERSION
    )]
    UnsupportedFormat {
        export: AbsPathBuf,
        export_type: String,
        version: String,
    },
    #[error(
        "the region at {}:{} of the function `{function}` in the coverage export `{export}` is of kind {kind}, which is not a code region",
        .start.line,
        .start.column
    )]
    UnsupportedRegionKind {
        export: AbsPathBuf,
        function: String,
        kind: u64,
        start: RegionPosition,
    },
}

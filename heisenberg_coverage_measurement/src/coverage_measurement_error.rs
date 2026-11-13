use std::io;
use std::time::SystemTime;

use humantime::format_rfc3339_nanos;
use ra_ap_hir::HirFileId;
use ra_ap_hir::HirFilePosition;
use ra_ap_hir_expand::ExpandError;
use ra_ap_paths::AbsPathBuf;
use ra_ap_syntax::SyntaxError;
use ra_ap_syntax::TextSize;
use ra_ap_vfs::FileId;
use thiserror::Error;

use heisenberg_coverage_export::region_position::RegionPosition;
use heisenberg_member_crates::member_crates_error::MemberCratesError;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_source_locator::source_locator_error::SourceLocatorError;

#[derive(Debug, Error)]
pub enum CoverageMeasurementError {
    #[error("rust-analyzer expands an attribute macro into {0:?}, which is no macro file")]
    AttributeExpansionWithoutMacroCall(HirFileId),
    #[error(
        "the macro invocation at {call} fails to expand under rust-analyzer, so the functions it writes cannot be measured: {message}"
    )]
    FailedMacroExpansion {
        call: SourceLocation,
        error: ExpandError,
        message: String,
    },
    #[error(
        "rust-analyzer parsed the body of a function at offset {offset:?} of {file_id:?} without its braces"
    )]
    FunctionBodyWithoutBraces {
        file_id: HirFileId,
        offset: TextSize,
    },
    #[error("rust-analyzer parsed a function at offset {offset:?} of {file_id:?} without its name")]
    FunctionWithoutName {
        file_id: HirFileId,
        offset: TextSize,
    },
    #[error(
        "rust-analyzer parsed a macro call without a path at offset {:?} of {:?}",
        .0.offset,
        .0.file_id
    )]
    MacroCallWithoutPath(HirFilePosition),
    #[error(transparent)]
    MemberCrates(#[from] MemberCratesError),
    #[error(
        "the coverage export `{export}`, modified at {}, is older than `{file}`, modified at {}; export the coverage of a run of the current sources",
        format_rfc3339_nanos(*.export_modified),
        format_rfc3339_nanos(*.file_modified)
    )]
    OutdatedExport {
        export: AbsPathBuf,
        export_modified: SystemTime,
        file: AbsPathBuf,
        file_modified: SystemTime,
    },
    #[error(
        "the attribute at {attribute} is active only in the build without cfg(test), which rust-analyzer does not analyse for this crate target, so the code it applies to cannot be measured; express its condition positively"
    )]
    ProductionOnlyAttribute { attribute: SourceLocation },
    #[error(
        "the code at {code} is active only in the build without cfg(test), which rust-analyzer does not analyse for this crate target, so it cannot be measured; express its condition positively"
    )]
    ProductionOnlyCode { code: SourceLocation },
    #[error(
        "the coverage export `{export}` records regions in `{file}`, a module file that only benchmark targets include; benchmarks never run in the test run, so export the coverage of a run that builds no benchmark target"
    )]
    RegionInBenchmarkTarget {
        export: AbsPathBuf,
        file: AbsPathBuf,
    },
    #[error(
        "the coverage export `{export}` records regions in `{file}`, a file of a workspace member that no crate target of the analysed configuration includes as a module file; export the coverage of a run of the analysed configuration"
    )]
    RegionInFileOutsideModuleTrees {
        export: AbsPathBuf,
        file: AbsPathBuf,
    },
    #[error(
        "the coverage export `{export}` records a region at {}:{} of `{file}`, in code that the analysed configuration leaves inactive; export the coverage of a run of the analysed configuration, and make production code independent of cfg(coverage), which cargo llvm-cov sets unless it runs with --no-cfg-coverage",
        .start.line,
        .start.column
    )]
    RegionInInactiveCode {
        export: AbsPathBuf,
        file: AbsPathBuf,
        start: RegionPosition,
    },
    #[error(
        "the measurement attributes a region to the function whose body opens at offset {offset:?} of {file_id:?}, which it does not measure"
    )]
    RegionOfAnUnmeasuredFunction { file_id: FileId, offset: TextSize },
    #[error(
        "the coverage export `{export}` records a region from {}:{} to {}:{} of `{file}`, which the current text of the file does not hold; export the coverage of a run of the current sources",
        .start.line,
        .start.column,
        .end.line,
        .end.column
    )]
    RegionOutsideFileText {
        end: RegionPosition,
        export: AbsPathBuf,
        file: AbsPathBuf,
        start: RegionPosition,
    },
    #[error(transparent)]
    SourceLocator(#[from] SourceLocatorError),
    #[error("the metadata of `{}` cannot be read", .path)]
    SourceMetadataUnreadable {
        path: AbsPathBuf,
        #[source]
        source: io::Error,
    },
    #[error(
        "the expansion of the macro invocation at {} does not parse under rust-analyzer, so the functions it writes cannot be measured",
        .location
    )]
    UnparsableMacroExpansion {
        location: SourceLocation,
        #[source]
        source: SyntaxError,
    },
    #[error(
        "rust-analyzer cannot resolve the attribute or derive at {attribute}, so the code it applies to cannot be measured"
    )]
    UnresolvedAttribute { attribute: SourceLocation },
    #[error(
        "rust-analyzer cannot resolve the macro of the call at {call}, so the functions it writes cannot be measured"
    )]
    UnresolvedMacroCall { call: SourceLocation },
}

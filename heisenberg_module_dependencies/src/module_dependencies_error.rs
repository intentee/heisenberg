use ra_ap_hir::Impl;
use ra_ap_syntax::TextSize;
use ra_ap_vfs::FileId;
use thiserror::Error;

use heisenberg_task_graph::task_graph_error::TaskGraphError;

#[derive(Debug, Error)]
pub enum ModuleDependenciesError {
    #[error(
        "rust-analyzer locates offset {offset:?} in file {file_id:?}, which its line index does not hold"
    )]
    InvalidSourceOffset { file_id: FileId, offset: TextSize },
    #[error("name resolution cannot be scheduled over the crate dependency graph")]
    NameResolutionSchedule(#[source] TaskGraphError),
    #[error("rust-analyzer refers to file {0:?}, which the loaded source files do not hold")]
    UnknownSourceFile(FileId),
    #[error("rust-analyzer gives no source for the impl {0:?} that a `Self` resolves to")]
    SelfTypeWithoutSource(Impl),
}

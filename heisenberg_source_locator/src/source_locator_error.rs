use ra_ap_syntax::TextSize;
use ra_ap_vfs::FileId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SourceLocatorError {
    #[error(
        "rust-analyzer locates offset {offset:?} in file {file_id:?}, which its line index does not hold"
    )]
    InvalidSourceOffset { file_id: FileId, offset: TextSize },
    #[error("rust-analyzer refers to file {0:?}, which the loaded source files do not hold")]
    UnknownSourceFile(FileId),
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AbsolutePathError {
    #[error("`{path}` is not an absolute path")]
    RelativePath { path: String },
}

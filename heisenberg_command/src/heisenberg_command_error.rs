use std::io;

use heisenberg_project_model::project_model_error::ProjectModelError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HeisenbergCommandError {
    #[error("the project cannot be loaded")]
    ProjectModel(#[from] ProjectModelError),
    #[error("`{path}` is not an absolute path")]
    RelativePath { path: String },
    #[error("the report cannot be written")]
    WriteReport(#[source] io::Error),
}

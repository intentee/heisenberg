use std::io;

use thiserror::Error;

use heisenberg_analysis_threads::analysis_threads_error::AnalysisThreadsError;
use heisenberg_analysis_threads::panic_payload::PanicPayload;
use heisenberg_error_context::contextual_error::ContextualError;
use heisenberg_module_dependencies::module_dependencies_error::ModuleDependenciesError;
use heisenberg_project_model::project_model_error::ProjectModelError;
use heisenberg_project_model::supported_target::SupportedTarget;

#[derive(Debug, Error)]
pub enum HeisenbergCommandError {
    #[error("the analysis thread for {} panicked", .0.context.triple())]
    AnalysisPanicked(#[source] ContextualError<SupportedTarget, PanicPayload>),
    #[error("the analysis thread for {} cannot be started", .0.context.triple())]
    AnalysisThread(#[source] ContextualError<SupportedTarget, AnalysisThreadsError>),
    #[error("the module dependencies cannot be extracted for {}", .0.context.triple())]
    ModuleDependencies(#[source] ContextualError<SupportedTarget, ModuleDependenciesError>),
    #[error("the project cannot be loaded")]
    ProjectModel(#[from] ProjectModelError),
    #[error("the project cannot be loaded for {}", .0.context.triple())]
    TargetProjectModel(#[source] ContextualError<SupportedTarget, Box<ProjectModelError>>),
    #[error("the report cannot be written")]
    WriteReport(#[source] io::Error),
}

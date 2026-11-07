use std::io;

use thiserror::Error;

use heisenberg_analysis_threads::analysis_threads_error::AnalysisThreadsError;
use heisenberg_analysis_threads::panic_payload::PanicPayload;
use heisenberg_error_context::contextual_error::ContextualError;
use heisenberg_module_dependencies::module_dependencies_error::ModuleDependenciesError;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::project_model_error::ProjectModelError;

#[derive(Debug, Error)]
pub enum HeisenbergCommandError {
    #[error("the analysis thread for the configuration `{}` panicked", .0.context.name)]
    AnalysisPanicked(#[source] ContextualError<Configuration, PanicPayload>),
    #[error("the analysis thread for the configuration `{}` cannot be started", .0.context.name)]
    AnalysisThread(#[source] ContextualError<Configuration, AnalysisThreadsError>),
    #[error("the module dependencies cannot be extracted for the configuration `{}`", .0.context.name)]
    ModuleDependencies(#[source] ContextualError<Configuration, ModuleDependenciesError>),
    #[error("the project cannot be loaded")]
    ProjectModel(#[from] ProjectModelError),
    #[error("the project cannot be loaded for the configuration `{}`", .0.context.name)]
    ConfigurationProjectModel(#[source] ContextualError<Configuration, Box<ProjectModelError>>),
    #[error("the explanation cannot be written")]
    WriteExplanation(#[source] io::Error),
    #[error("the report cannot be written")]
    WriteReport(#[source] io::Error),
}

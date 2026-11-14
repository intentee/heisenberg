use std::error::Error as StdError;
use std::io;
use std::iter;

use thiserror::Error;

use heisenberg_analysis_threads::analysis_threads_error::AnalysisThreadsError;
use heisenberg_coverage_export::coverage_export_error::CoverageExportError;
use heisenberg_coverage_measurement::coverage_measurement_error::CoverageMeasurementError;
use heisenberg_module_dependencies::module_dependencies_error::ModuleDependenciesError;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::project_model_error::ProjectModelError;

#[derive(Debug, Error)]
pub enum HeisenbergCommandError {
    #[error("the analysis thread for the configuration `{}` cannot be started", .configuration.name)]
    AnalysisThread {
        configuration: Configuration,
        #[source]
        source: AnalysisThreadsError,
    },
    #[error("the run failed in {} ways", .0.len())]
    Failures(Vec<Self>),
    #[error("the coverage export cannot be used")]
    CoverageExport(#[source] CoverageExportError),
    #[error("the coverage of the configuration `{}` cannot be measured", .configuration.name)]
    CoverageMeasurement {
        configuration: Configuration,
        #[source]
        source: Box<CoverageMeasurementError>,
    },
    #[error("the module dependencies cannot be extracted for the configuration `{}`", .configuration.name)]
    ModuleDependencies {
        configuration: Configuration,
        #[source]
        source: Box<ModuleDependenciesError>,
    },
    #[error("the input that every analysed configuration shares cannot be read on its own thread")]
    PrerequisiteThread(#[source] AnalysisThreadsError),
    #[error("the project cannot be loaded")]
    ProjectModel(#[from] ProjectModelError),
    #[error("the project cannot be loaded for the configuration `{}`", .configuration.name)]
    ConfigurationProjectModel {
        configuration: Configuration,
        #[source]
        source: Box<ProjectModelError>,
    },
    #[error("the analysis cannot run on the worker pool")]
    WorkerPool(#[source] AnalysisThreadsError),
    #[error("the explanation cannot be written")]
    WriteExplanation(#[source] io::Error),
    #[error("the report cannot be written")]
    WriteReport(#[source] io::Error),
}

impl HeisenbergCommandError {
    pub(crate) fn chain(&self) -> Vec<String> {
        match self {
            Self::Failures(failures) => failures.iter().flat_map(Self::chain).collect(),
            _ => iter::successors(Some(self as &dyn StdError), |cause: &&dyn StdError| {
                (*cause).source()
            })
            .map(|cause| cause.to_string().trim_end().to_owned())
            .collect(),
        }
    }
}

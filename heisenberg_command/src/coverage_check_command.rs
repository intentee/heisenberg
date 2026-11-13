use std::collections::BTreeMap;
use std::ffi::OsString;

use clap::Args;

use heisenberg_absolute_path::absolute_path::AbsolutePath;
use heisenberg_coverage_export::coverage_export_file::CoverageExportFile;
use heisenberg_coverage_export::exported_coverage::ExportedCoverage;
use heisenberg_coverage_measurement::coverage_measurement::CoverageMeasurement;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::loaded_project::LoadedProject;
use heisenberg_project_model::resolved_project::ResolvedProject;

use crate::configuration_coverage::ConfigurationCoverage;
use crate::configuration_pipeline::ConfigurationPipeline;
use crate::coverage_report::CoverageReport;
use crate::heisenberg_command_error::HeisenbergCommandError;
use crate::project_arguments::ProjectArguments;

#[derive(Args, Debug)]
pub struct CoverageCheckCommand {
    #[command(flatten)]
    pub project: ProjectArguments,
    #[arg(
        long,
        value_parser = AbsolutePath::parse,
        help = "Absolute path of the coverage export that `cargo llvm-cov report --json` writes"
    )]
    pub coverage_export: AbsolutePath,
}

impl CoverageCheckCommand {
    /// # Errors
    ///
    /// Returns every [`HeisenbergCommandError`] that kept the project from loading, the coverage
    /// export from being read or the selected configuration from being measured.
    pub fn execute(
        &self,
        environment: &BTreeMap<OsString, OsString>,
    ) -> Result<CoverageReport, HeisenbergCommandError> {
        self.run(&self.project.model(environment))
    }
}

impl ConfigurationPipeline for CoverageCheckCommand {
    type Analysis = ConfigurationCoverage;
    type Prerequisite = ExportedCoverage;
    type Report = CoverageReport;

    fn analysis(
        &self,
        configuration: &Configuration,
        project: &mut LoadedProject<'_>,
        coverage: &Self::Prerequisite,
    ) -> Result<Self::Analysis, HeisenbergCommandError> {
        CoverageMeasurement { coverage, project }
            .measure()
            .map_err(Box::new)
            .map_err(|source| HeisenbergCommandError::CoverageMeasurement {
                configuration: configuration.clone(),
                source,
            })
            .map(|coverage| ConfigurationCoverage {
                configuration: configuration.clone(),
                coverage,
            })
    }

    fn prerequisite(
        &self,
        ResolvedProject { manifest, .. }: &ResolvedProject<'_>,
    ) -> Result<Self::Prerequisite, HeisenbergCommandError> {
        CoverageExportFile {
            path: &self.coverage_export.path,
            workspace_manifest: manifest,
        }
        .read()
        .map_err(HeisenbergCommandError::CoverageExport)
    }

    fn report(
        &self,
        ResolvedProject { scope, .. }: &ResolvedProject<'_>,
        measurements: Vec<Self::Analysis>,
        _: &Self::Prerequisite,
    ) -> Self::Report {
        CoverageReport {
            measurements,
            scope: scope.clone(),
        }
    }
}
